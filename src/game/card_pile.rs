use itertools::Itertools;

use super::cards::Card;

pub struct CardPile {
    cards: Vec<Card>,
}

#[derive(Debug)]
pub enum DrawResult {
    Complete,
    Partial(usize),
}

impl CardPile {
    pub fn new() -> Self {
        CardPile { cards: vec![] }
    }

    /** Try to move `n` cards, and return whether we were successful */
    pub fn move_n_to(&mut self, n: usize, other: &mut CardPile) -> DrawResult {
        let len = self.cards.len();
        let index = len.saturating_sub(n);
        let count = len - index;

        let cards = self.cards.drain(index..);
        other.cards.extend(cards);

        if count == n {
            DrawResult::Complete
        } else {
            DrawResult::Partial(n - count)
        }
    }

    /** Try to move `n` cards, and just move fewer if there weren't enough */
    pub fn move_up_to_n_to(&mut self, n: usize, other: &mut CardPile) {
        let len = self.cards.len();
        let index = len.saturating_sub(n);

        let cards = self.cards.drain(index..);
        other.cards.extend(cards);
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

    pub fn temp_internal_vec(&mut self) -> &mut Vec<Card> {
        // TODO: replace usages with proper CardPile methods
        &mut self.cards
    }

    pub fn temp_iter(&self) -> impl Iterator<Item = &Card> {
        self.cards.iter()
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
    use crate::game::cards::{CardNames, Cards};

    #[test]
    fn added_cards_can_be_drawn() {
        let mut deck = CardPile::new();
        deck.add_range(&mut vec![Cards::copper()]);
        deck.add_range(&mut vec![Cards::silver()]);
        deck.add_range(&mut vec![Cards::gold()]);

        let mut other = CardPile::new();

        // since the cards were added one at a time, they get drawn in reverse order
        assert_eq!(CardNames::GOLD, deck.peek().unwrap().name);
        let _ = deck.move_n_to(1, &mut other);
        assert_eq!(CardNames::SILVER, deck.peek().unwrap().name);
        let _ = deck.move_n_to(1, &mut other);
        assert_eq!(CardNames::COPPER, deck.peek().unwrap().name);
    }

    #[test]
    fn if_there_arent_enough_cards_then_remaining_cards_get_drawn() {
        let mut deck = CardPile::new();
        deck.add_range(&mut vec![Cards::copper(), Cards::silver(), Cards::gold()]);
        let mut other = CardPile::new();

        let result = deck.move_n_to(5, &mut other);

        assert!(matches!(result, DrawResult::Partial(2)));
    }
}
