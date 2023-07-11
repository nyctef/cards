use core::num;
use itertools::Itertools;
use std::cmp::min;

use super::model::Card;

pub struct CardPile {
    cards: Vec<Card>,
}

#[derive(Debug)]
pub enum DrawResult {
    Complete(Vec<Card>),
    Partial(Vec<Card>, usize),
}

impl CardPile {
    pub fn new() -> Self {
        CardPile { cards: vec![] }
    }

    /** Try to draw `n` cards, and return whether we were successful */
    pub fn take_n(&mut self, n: usize) -> DrawResult {
        let cards = self.take_up_to_n(n);
        if cards.len() == n {
            DrawResult::Complete(cards)
        } else {
            let remaining = n - &cards.len();
            DrawResult::Partial(cards, remaining)
        }
    }

    /** Try to draw `n` cards, and just return fewer if there weren't enough */
    pub fn take_up_to_n(&mut self, n: usize) -> Vec<Card> {
        let index = self.cards.len().saturating_sub(n);
        self.cards.split_off(index)
    }

    pub fn add_at_top(&mut self, card: Card) {
        self.cards.insert(0, card)
    }

    pub fn add_range(&mut self, cards: &mut Vec<Card>) {
        self.cards.append(cards)
    }

    pub fn peek(&self) -> Option<&Card> {
        self.cards.last()
    }

    pub fn take_all(&mut self) -> Vec<Card> {
        self.cards.drain(..).collect()
    }
}

impl From<Vec<Card>> for CardPile {
    fn from(value: Vec<Card>) -> Self {
        CardPile { cards: value }
    }
}

impl std::fmt::Debug for CardPile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let runs = &self.cards.iter().group_by(|c| c.name);

        f.debug_list()
            .entries(
                runs.into_iter()
                    .map(|(k, g)| format!("{} {:?}", g.count(), k)),
            )
            .finish()
    }
}

/* TODO
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn added_cards_can_be_drawn() {
        let mut deck = CardPile::new();
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
        let mut deck = CardPile::new();
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

*/
