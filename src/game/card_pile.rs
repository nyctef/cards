use itertools::Itertools;

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
            let remaining = n - cards.len();
            DrawResult::Partial(cards, remaining)
        }
    }

    /** Try to draw `n` cards, and just return fewer if there weren't enough */
    pub fn take_up_to_n(&mut self, n: usize) -> Vec<Card> {
        let index = self.cards.len().saturating_sub(n);
        self.cards.split_off(index)
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

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::model::{CardNames, Cards};

    #[test]
    fn added_cards_can_be_drawn() {
        let mut deck = CardPile::new();
        deck.add_range(&mut vec![Cards::copper()]);
        deck.add_range(&mut vec![Cards::silver()]);
        deck.add_range(&mut vec![Cards::gold()]);

        // since the cards were added one at a time, they get drawn in reverse order
        assert_eq!(CardNames::GOLD, deck.peek().unwrap().name);
        let _ = deck.take_n(1);
        assert_eq!(CardNames::SILVER, deck.peek().unwrap().name);
        let _ = deck.take_n(1);
        assert_eq!(CardNames::COPPER, deck.peek().unwrap().name);
    }

    #[test]
    fn if_there_arent_enough_cards_then_remaining_cards_get_drawn() {
        let mut deck = CardPile::new();
        deck.add_range(&mut vec![Cards::copper(), Cards::silver(), Cards::gold()]);

        let cards = deck.take_n(5);

        assert!(matches!(cards, DrawResult::Partial(_, 2)));
    }
}
