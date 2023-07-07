// todo: not sure about the naming or structure here yet

use super::deck::Deck;

pub struct PlayArea<C> {
    deck: Deck<C>,
    hand: Vec<C>,
}

impl<C> PlayArea<C>
where
    C: std::fmt::Debug,
{
    fn new() -> Self {
        PlayArea {
            deck: Deck::<C>::new(),
            hand: vec![],
        }
    }

    pub fn from_initial_cards(mut cards: Vec<C>) -> Self {
        let mut result = Self::new();
        result.deck.add_range(&mut cards);
        result
    }

    pub fn draw_hand(&mut self) {
        let mut cards = self.deck.draw(5);
        match cards {
            super::deck::DrawResult::Complete(mut cards) => self.hand.append(&mut cards),
            super::deck::DrawResult::Partial(mut cards, remaining) => self.hand.append(&mut cards),
        }
    }

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
}
