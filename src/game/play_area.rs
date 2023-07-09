// todo: not sure about the naming or structure here yet

use crate::logs::{GameEvent, GameLog};

use super::card_pile::{CardPile, DrawResult};

#[derive(Debug)]
pub struct PlayArea<C> {
    deck: CardPile<C>,
    hand: Vec<C>,
    discard: Vec<C>,
}

impl<C> PlayArea<C>
where
    C: std::fmt::Debug,
{
    pub fn new() -> Self {
        PlayArea {
            deck: CardPile::<C>::new(),
            hand: vec![],
            discard: vec![],
        }
    }

    pub fn from_initial_cards(mut cards: Vec<C>) -> Self {
        let mut result = Self::new();
        // todo: shuffle
        result.deck.add_range(&mut cards);
        result
    }

    pub fn draw_hand(&mut self, log: &dyn GameLog) {
        let mut cards = self.deck.take_n(5);
        match cards {
            DrawResult::Complete(mut cards) => {
                self.hand.append(&mut cards);
                log.record(GameEvent::Todo(format!("draws 5 cards")))
            }
            DrawResult::Partial(mut cards, remaining) => {
                log.record(GameEvent::Todo(format!(
                    "draws {} cards, shuffles their deck, and draws {} more",
                    cards.len(),
                    remaining
                )));
                self.hand.append(&mut cards);
                // we didn't get all the cards we need, so shuffle the discard pile
                // and turn it back into the deck:
                // todo: shuffle
                self.deck.add_range(&mut self.discard);
                let mut remaining_cards = self.deck.take_up_to_n(remaining);
                self.hand.append(&mut remaining_cards)
            }
        }
    }

    pub fn discard_hand(&mut self) {
        self.discard.append(&mut self.hand);
    }

    pub fn gain_cards_to_discard_pile(&mut self, cards: &mut Vec<C>) {
        self.discard.append(cards)
    }

    pub fn gain_card_to_discard_pile(&mut self, card: C) {
        self.discard.push(card)
    }

    pub fn inspect_hand(&mut self) -> &Vec<C> {
        &self.hand
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logs::tests::TestLog;

    #[test]
    fn drawn_cards_go_into_hand() {
        let mut play_area =
            PlayArea::<i32>::from_initial_cards(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let log = TestLog::new();
        play_area.draw_hand(&log);
        assert_eq!(&vec![6, 7, 8, 9, 10], play_area.inspect_hand());
    }

    #[test]
    fn discarded_cards_leave_hand() {
        let mut play_area =
            PlayArea::<i32>::from_initial_cards(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let log = TestLog::new();
        play_area.draw_hand(&log);
        play_area.discard_hand();
        assert!(play_area.inspect_hand().is_empty());
    }

    #[test]
    fn discarded_cards_are_recycled_into_hand() {
        let mut play_area = PlayArea::<i32>::from_initial_cards(vec![1, 2, 3, 4, 5, 6, 7]);
        // draw 5 and discard
        let log = TestLog::new();
        play_area.draw_hand(&log);
        play_area.discard_hand();
        // attempt to draw another 5: get some of the original discarded cards
        play_area.draw_hand(&log);
        assert_eq!(&vec![1, 2, 5, 6, 7], play_area.inspect_hand());
    }

    #[test]
    fn can_attempt_to_draw_five_even_if_deck_contains_fewer_cards() {
        let mut play_area = PlayArea::<i32>::from_initial_cards(vec![1, 2, 3]);
        let log = TestLog::new();
        play_area.draw_hand(&log);
        assert_eq!(&vec![1, 2, 3], play_area.inspect_hand());
    }
}
