use core::num;
use std::cmp::min;

#[derive(Debug)]
pub struct Deck<C> {
    cards: Vec<C>,
}

#[derive(Debug, PartialEq)]
pub enum DrawResult<C> {
    Complete(Vec<C>),
    Partial(Vec<C>, usize),
}

impl<C> Deck<C>
where
    C: std::fmt::Debug,
{
    pub fn new() -> Self {
        Deck { cards: vec![] }
    }

    pub fn draw(&mut self, num_cards_requested: usize) -> DrawResult<C> {
        let index = self.cards.len().saturating_sub(num_cards_requested);
        let cards = self.cards.split_off(index);
        if cards.len() == num_cards_requested {
            DrawResult::Complete(cards)
        } else {
            let remaining = num_cards_requested - &cards.len();
            DrawResult::Partial(cards, remaining)
        }
    }

    pub fn add_at_top(&mut self, card: C) {
        self.cards.insert(0, card)
    }

    pub fn add_range(&mut self, cards: &mut Vec<C>) {
        self.cards.append(cards)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn added_cards_can_be_drawn() {
        let mut deck = Deck::<i32>::new();
        deck.add_at_top(1);
        deck.add_at_top(2);
        deck.add_at_top(3);
        let cards = deck.draw(3);
        assert_eq!(
            DrawResult::Complete(vec![3, 2, 1]),
            cards,
            "Since each card was added to the top, they get drawn in reverse order"
        )
    }

    #[test]
    fn if_there_arent_enough_cards_then_remaining_cards_get_drawn() {
        let mut deck = Deck::<i32>::new();
        deck.add_range(&mut vec![1, 2, 3]);
        let cards = deck.draw(5);
        assert_eq!(
            cards,
            DrawResult::Partial {
                0: vec![1, 2, 3],
                1: 2
            }
        )
    }
}
