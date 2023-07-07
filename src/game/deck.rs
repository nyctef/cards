use core::num;
use std::cmp::min;

pub struct Deck<C> {
    cards: Vec<C>,
}

impl<C> Deck<C>
where
    C: std::fmt::Debug,
{
    pub fn new() -> Self {
        Deck { cards: vec![] }
    }

    fn draw(&mut self, num_cards: usize) -> Vec<C> {
        let index = self.cards.len().saturating_sub(num_cards);
        self.cards.split_off(index)
    }

    fn add_at_top(&mut self, card: C) {
        self.cards.insert(0, card)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drawing_from_an_empty_deck_produces_zero_cards() {
        let mut deck = Deck::<i32>::new();
        let cards = deck.draw(5);
        assert_eq!(0, cards.len());
    }

    #[test]
    fn added_cards_can_be_drawn() {
        let mut deck = Deck::<i32>::new();
        deck.add_at_top(1);
        deck.add_at_top(2);
        deck.add_at_top(3);
        let cards = deck.draw(3);
        assert_eq!(
            vec![3, 2, 1],
            cards,
            "Since each card was added to the top, they get drawn in reverse order"
        )
    }
}
