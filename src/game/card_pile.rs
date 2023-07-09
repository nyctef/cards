use core::num;
use std::cmp::min;

pub struct CardPile<C> {
    cards: Vec<C>,
}

#[derive(Debug, PartialEq)]
pub enum DrawResult<C> {
    Complete(Vec<C>),
    Partial(Vec<C>, usize),
}

impl<C> CardPile<C>
where
    C: std::fmt::Debug,
{
    pub fn new() -> Self {
        CardPile { cards: vec![] }
    }

    /** Try to draw `n` cards, and return whether we were successful */
    pub fn take_n(&mut self, n: usize) -> DrawResult<C> {
        let cards = self.take_up_to_n(n);
        if cards.len() == n {
            DrawResult::Complete(cards)
        } else {
            let remaining = n - &cards.len();
            DrawResult::Partial(cards, remaining)
        }
    }

    /** Try to draw `n` cards, and just return fewer if there weren't enough */
    pub fn take_up_to_n(&mut self, n: usize) -> Vec<C> {
        let index = self.cards.len().saturating_sub(n);
        self.cards.split_off(index)
    }

    pub fn add_at_top(&mut self, card: C) {
        self.cards.insert(0, card)
    }

    pub fn add_range(&mut self, cards: &mut Vec<C>) {
        self.cards.append(cards)
    }

    pub fn peek(&self) -> Option<&C> {
        self.cards.last()
    }
}

impl<C> From<Vec<C>> for CardPile<C> {
    fn from(value: Vec<C>) -> Self {
        CardPile { cards: value }
    }
}

impl<C> std::fmt::Debug for CardPile<C>
where
    C: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(&self.cards).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn added_cards_can_be_drawn() {
        let mut deck = CardPile::<i32>::new();
        deck.add_at_top(1);
        deck.add_at_top(2);
        deck.add_at_top(3);
        let cards = deck.take_n(3);
        assert_eq!(
            DrawResult::Complete(vec![3, 2, 1]),
            cards,
            "Since each card was added to the top, they get drawn in reverse order"
        )
    }

    #[test]
    fn if_there_arent_enough_cards_then_remaining_cards_get_drawn() {
        let mut deck = CardPile::<i32>::new();
        deck.add_range(&mut vec![1, 2, 3]);
        let cards = deck.take_n(5);
        assert_eq!(
            cards,
            DrawResult::Partial {
                0: vec![1, 2, 3],
                1: 2
            }
        )
    }
}
