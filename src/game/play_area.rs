// todo: not sure about the naming or structure here yet

use super::deck::{Deck, DrawResult};

pub struct PlayArea<C> {
    deck: Deck<C>,
    hand: Vec<C>,
    discard: Vec<C>,
}

impl<C> PlayArea<C>
where
    C: std::fmt::Debug,
{
    fn new() -> Self {
        PlayArea {
            deck: Deck::<C>::new(),
            hand: vec![],
            discard: vec![],
        }
    }

    pub fn from_initial_cards(mut cards: Vec<C>) -> Self {
        let mut result = Self::new();
        result.deck.add_range(&mut cards);
        result
    }

    fn ignore_draw_result(res: DrawResult<C>) -> Vec<C> {
        match res {
            DrawResult::Complete(c) => c,
            DrawResult::Partial(c, _) => c,
        }
    }

    pub fn draw_hand(&mut self) {
        let mut cards = self.deck.draw(5);
        match cards {
            DrawResult::Complete(mut cards) => self.hand.append(&mut cards),
            DrawResult::Partial(mut cards, remaining) => {
                self.hand.append(&mut cards);
                // we didn't get all the cards we need, so shuffle the discard pile
                // and turn it back into the deck:
                // todo: shuffle
                self.deck.add_range(&mut self.discard);
                let mut remaining_cards = Self::ignore_draw_result(self.deck.draw(remaining));
                self.hand.append(&mut remaining_cards)
            }
        }
    }

    pub fn discard_hand(&mut self) {
        self.discard.append(&mut self.hand);
    }

    #[cfg(test)]
    pub fn debug_inspect_hand(&mut self) -> &Vec<C> {
        &self.hand
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drawn_cards_go_into_hand() {
        let mut play_area =
            PlayArea::<i32>::from_initial_cards(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        play_area.draw_hand();
        assert_eq!(&vec![6, 7, 8, 9, 10], play_area.debug_inspect_hand());
    }

    #[test]
    fn discarded_cards_leave_hand() {
        let mut play_area =
            PlayArea::<i32>::from_initial_cards(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        play_area.draw_hand();
        play_area.discard_hand();
        assert!(play_area.debug_inspect_hand().is_empty());
    }

    #[test]
    fn discarded_cards_are_recycled_into_hand() {
        let mut play_area = PlayArea::<i32>::from_initial_cards(vec![1, 2, 3, 4, 5, 6, 7]);
        // draw 5 and discard
        play_area.draw_hand();
        play_area.discard_hand();
        // attempt to draw another 5: get some of the original discarded cards
        play_area.draw_hand();
        assert_eq!(&vec![1, 2, 5, 6, 7], play_area.debug_inspect_hand());
    }

    #[test]
    fn can_attempt_to_draw_five_even_if_deck_contains_fewer_cards() {
        let mut play_area = PlayArea::<i32>::from_initial_cards(vec![1, 2, 3]);
        play_area.draw_hand();
        assert_eq!(&vec![1, 2, 3], play_area.debug_inspect_hand());
    }
}
