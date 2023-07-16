use super::{
    card_pile::{CardPile, DrawResult},
    cards::{Card, CardName},
    player_counters::PlayerCounters,
};
use crate::{
    game::logs::{GameEvent, GameLog},
    game::shuffler::Shuffler,
};

#[derive(Debug)]
pub struct PlayArea<'a> {
    deck: CardPile,
    hand: Vec<Card>,
    in_play: Vec<Card>,
    discard: Vec<Card>,
    shuffler: &'a dyn Shuffler<Card>,
}

impl<'p> PlayArea<'p> {
    pub fn new(shuffler: &'p dyn Shuffler<Card>) -> Self {
        PlayArea {
            deck: CardPile::new(),
            hand: vec![],
            in_play: vec![],
            discard: vec![],
            shuffler,
        }
    }

    pub fn draw_hand(&mut self, log: &GameLog) {
        let cards = self.deck.take_n(5);
        match cards {
            DrawResult::Complete(mut cards) => {
                self.hand.append(&mut cards);
                log.record(GameEvent::DrawCards(5));
            }
            DrawResult::Partial(mut cards, remaining) => {
                log.record(GameEvent::DrawCards(cards.len()));
                self.hand.append(&mut cards);
                // we didn't get all the cards we need, so shuffle the discard pile
                // and turn it back into the deck:
                assert!(self.deck.is_empty() && self.in_play.is_empty());

                log.record(GameEvent::Shuffle());
                let mut shuffled = self.shuffler.shuffle(&mut self.discard);

                self.deck.add_range(&mut shuffled);
                log.record(GameEvent::DrawCards(remaining));
                let mut remaining_cards = self.deck.take_up_to_n(remaining);
                self.hand.append(&mut remaining_cards)
            }
        }
    }

    pub fn discard_hand(&mut self) {
        self.discard.append(&mut self.hand);
    }

    pub fn discard_in_play(&mut self) {
        self.discard.append(&mut self.in_play);
    }

    pub fn gain_cards_to_discard_pile(&mut self, cards: &mut Vec<Card>) {
        self.discard.append(cards)
    }

    pub fn gain_card_to_discard_pile(&mut self, card: Card) {
        self.discard.push(card)
    }

    pub fn inspect_hand(&self) -> impl Iterator<Item = &Card> + '_ {
        self.hand.iter()
    }

    pub fn play_card(&mut self, name: CardName, counters: &mut PlayerCounters) {
        let card = self.hand.remove(
            self.hand
                .iter()
                .position(|c| c.name == name)
                .expect("BUG: expected hand to contain card being played"),
        );

        match card.effect {
            super::effects::CardEffect::None => {}
            super::effects::CardEffect::Sequence(_) => todo!(),
            super::effects::CardEffect::AddActions(_) => todo!(),
            super::effects::CardEffect::AddBuys(_) => todo!(),
            super::effects::CardEffect::AddCoins(treasure_value) => {
                counters.coins += treasure_value
            }
            super::effects::CardEffect::DrawCards(_) => todo!(),
            super::effects::CardEffect::TrashCardsFromHand(_) => todo!(),
        }

        self.in_play.push(card);
    }

    pub fn take_all_cards(&mut self) -> Vec<Card> {
        let mut res = vec![];
        res.append(&mut self.deck.take_all());
        res.append(&mut self.hand);
        res.append(&mut self.discard);
        res
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use std::rc::Rc;

    use super::*;
    use crate::game::{
        cards::{CardNames, Cards},
        logs::tests::TestLog,
        shuffler::NoShuffle,
    };

    macro_rules! cards {
        ( $($c:ident $n:expr) ; +) => {
            {
                // based on https://danielkeep.github.io/tlborm/book/mbe-macro-rules.html
                let mut v = Vec::new();
                $(v.append(&mut (0..$n).map(|_| Cards::$c()).collect_vec());)+
                v
            }
        };
    }

    macro_rules! names {
        ( $($c:ident $n:expr) ; +) => {
            {
                // based on https://danielkeep.github.io/tlborm/book/mbe-macro-rules.html
                let mut v = Vec::new();
                $(v.extend_from_slice(&[CardNames::$c; $n]);)+
                v
            }
        };
    }

    fn from_initial_cards(mut cards: Vec<Card>) -> PlayArea<'static> {
        let shuffler = Box::leak(Box::new(NoShuffle::new()));
        let mut area = PlayArea::new(shuffler);
        area.gain_cards_to_discard_pile(&mut cards);
        area
    }

    fn standard_cards() -> Vec<Card> {
        cards![copper 7; estate 3]
    }

    fn make_log() -> GameLog {
        GameLog::new(Rc::new(TestLog::new()))
    }

    #[test]
    fn drawn_cards_go_into_hand() {
        let cards = standard_cards();
        let mut play_area = from_initial_cards(cards);
        play_area.draw_hand(&make_log());
        assert_eq!(
            names![COPPER 2; ESTATE 3],
            play_area.inspect_hand().map(|c| c.name).collect_vec()
        );
    }

    #[test]
    fn discarded_cards_leave_hand() {
        let mut play_area = from_initial_cards(standard_cards());
        play_area.draw_hand(&make_log());
        play_area.discard_hand();
        assert_eq!(0, play_area.inspect_hand().count());
    }

    #[test]
    fn discarded_cards_are_recycled_into_hand() {
        let cards = cards![copper 5; estate 2];

        let mut play_area = from_initial_cards(cards);
        // draw 5 and discard
        play_area.draw_hand(&make_log());
        play_area.discard_hand();
        // attempt to draw another 5: get some of the original discarded cards
        play_area.draw_hand(&make_log());
        assert_eq!(
            names![COPPER 3; ESTATE 2],
            play_area.inspect_hand().map(|c| c.name).collect_vec()
        );
    }

    #[test]
    fn can_attempt_to_draw_five_even_if_deck_contains_fewer_cards() {
        let cards = cards![copper 3];

        let mut play_area = from_initial_cards(cards);
        play_area.draw_hand(&make_log());

        assert_eq!(3, play_area.inspect_hand().count());
    }
}
