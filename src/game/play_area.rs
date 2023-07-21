use super::{
    card_pile::{CardPile, DrawResult},
    cards::{Card, CardName},
    effects::CardEffect,
    logs::{GameEvent, GameLog},
    player_counters::PlayerCounters,
    shuffler::Shuffler,
};

#[derive(Debug)]
pub struct PlayArea<'a> {
    deck: CardPile,
    hand: CardPile,
    in_play: Vec<Card>,
    discard: CardPile,
    shuffler: &'a dyn Shuffler<Card>,
}

impl<'p> PlayArea<'p> {
    pub fn new(shuffler: &'p dyn Shuffler<Card>) -> Self {
        PlayArea {
            deck: CardPile::with_initial_capacity(20),
            hand: CardPile::with_initial_capacity(5),
            in_play: vec![],
            discard: CardPile::with_initial_capacity(20),
            shuffler,
        }
    }

    #[cfg(test)]
    pub fn test_from_hand(hand: Vec<Card>) -> Self {
        PlayArea {
            deck: CardPile::new(),
            hand: CardPile::from(hand),
            in_play: vec![],
            discard: CardPile::new(),
            shuffler: &crate::game::shuffler::NoShuffle,
        }
    }

    pub fn draw_n(&mut self, n: usize, log: &GameLog) {
        let status = self.deck.move_n_to(n, &mut self.hand);
        match status {
            DrawResult::Complete => {
                log.record(GameEvent::DrawCards(n));
            }
            DrawResult::Partial(remaining) => {
                log.record(GameEvent::DrawCards(n - remaining));
                // we didn't get all the cards we need, so shuffle the discard pile
                // and turn it back into the deck:
                assert!(self.deck.is_empty());

                log.record(GameEvent::Shuffle());
                self.shuffler.shuffle(&mut self.discard);
                self.discard.move_all_to(&mut self.deck);

                // todo: fix log if we didn't actually get `remaining` cards back
                log.record(GameEvent::DrawCards(remaining));
                self.deck.move_up_to_n_to(remaining, &mut self.hand);
            }
        }
    }

    pub fn draw_hand(&mut self, log: &GameLog) {
        self.draw_n(5, log)
    }

    pub fn discard_hand(&mut self) {
        self.hand.move_all_to(&mut self.discard);
    }

    pub fn discard_in_play(&mut self) {
        self.discard.temp_internal_vec().append(&mut self.in_play);
    }

    pub fn gain_cards_to_discard_pile(&mut self, cards: &mut Vec<Card>) {
        self.discard.temp_internal_vec().append(cards)
    }

    pub fn gain_card_to_discard_pile(&mut self, card: Card) {
        self.discard.temp_internal_vec().push(card)
    }

    pub fn inspect_hand(&self) -> impl Iterator<Item = &Card> + '_ {
        self.hand.temp_iter()
    }

    pub fn play_card(&mut self, name: CardName, counters: &mut PlayerCounters, log: &GameLog) {
        let hand = &mut self.hand.temp_internal_vec();
        let card = hand.swap_remove(
            hand.iter()
                .position(|c| c.name == name)
                .expect("BUG: expected hand to contain card being played"),
        );

        self.resolve_effect(&card.effect, counters, log);

        self.in_play.push(card);
    }

    fn resolve_effect(
        &mut self,
        effect: &CardEffect,
        counters: &mut PlayerCounters,
        log: &GameLog,
    ) {
        match effect {
            CardEffect::None => {}
            CardEffect::Sequence(s) => s.iter().for_each(|e| self.resolve_effect(e, counters, log)),
            CardEffect::AddActions(a) => counters.actions += a,
            // CardEffect::AddBuys(_) => todo!(),
            CardEffect::AddCoins(c) => counters.coins += c,
            CardEffect::DrawCards(n) => self.draw_n(*n as usize, log),
            // CardEffect::TrashCardsFromHand(_) => todo!(),
        }
    }

    pub fn take_all_cards(&mut self) -> Vec<Card> {
        let mut res = Vec::with_capacity(
            self.deck.temp_internal_vec().len()
                + self.hand.temp_internal_vec().len()
                + self.discard.temp_internal_vec().len(),
        );
        res.append(&mut self.deck.take_all());
        res.append(&mut self.hand.temp_internal_vec());
        res.append(&mut self.discard.temp_internal_vec());
        res
    }
}

#[cfg(test)]
pub mod tests {
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

    // TODO: should probably find a better place for these macros to live
    pub(crate) use cards;
    pub(crate) use names;

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
        let mut play_area = from_initial_cards(standard_cards());

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
        let mut play_area = from_initial_cards(cards![copper 5; estate 2]);

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
        let mut play_area = from_initial_cards(cards![copper 3]);

        play_area.draw_hand(&make_log());

        assert_eq!(3, play_area.inspect_hand().count());
    }

    #[test]
    fn playing_treasure_increases_coins() {
        let mut play_area = PlayArea::test_from_hand(cards![copper 1; silver 1]);
        let mut counters = PlayerCounters::new_turn();

        play_area.play_card(CardNames::COPPER, &mut counters, &make_log());
        assert_eq!(1, counters.coins);
        play_area.play_card(CardNames::SILVER, &mut counters, &make_log());
        assert_eq!(3, counters.coins);
    }

    #[test]
    fn playing_smithy_draws_more_cards() {
        let mut play_area = PlayArea::test_from_hand(cards![smithy 1]);
        play_area.gain_cards_to_discard_pile(&mut cards![copper 3]);
        let mut counters = PlayerCounters::new_turn();

        play_area.play_card(CardNames::SMITHY, &mut counters, &make_log());
        assert_eq!(3, play_area.inspect_hand().count());
    }

    #[test]
    fn playing_village_increases_actions() {
        let mut play_area = PlayArea::test_from_hand(cards![village 1]);
        let mut counters = PlayerCounters::new_turn();

        assert_eq!(1, counters.actions);
        play_area.play_card(CardNames::VILLAGE, &mut counters, &make_log());
        // note these tests assume that play_area isn't responsible for decrementing actions
        assert_eq!(3, counters.actions);
    }
}
