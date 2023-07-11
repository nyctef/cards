use super::{
    card_pile::CardPile,
    model::{Card, CardName},
};

#[derive(Debug)]
pub struct Supply {
    supply_piles: Vec<CardPile>,
}

impl Supply {
    pub fn new() -> Self {
        Supply {
            supply_piles: vec![],
        }
    }

    pub fn buyable_cards(&self, coins: u8) -> impl Iterator<Item = CardName> + '_ {
        self.supply_piles
            .iter()
            .filter_map(move |s| s.peek().filter(|c| c.coins_cost <= coins).map(|c| c.name))
    }

    fn supply_pile_for(&mut self, card: CardName) -> Option<&mut CardPile> {
        self.supply_piles
            .iter_mut()
            .find(|s| s.peek().map(|c| c.name) == Some(card))
    }

    pub fn take_one(&mut self, card: CardName) -> Option<Card> {
        self.take_up_to_n(card, 1).into_iter().next()
    }

    pub fn add(&mut self, vec: Vec<Card>) {
        self.supply_piles.push(vec.into());
    }

    pub fn take_up_to_n(&mut self, card: CardName, n: usize) -> Vec<Card> {
        let pile = self
            .supply_pile_for(card)
            .expect(format!("Missing supply pile for {:?}", card).as_str());
        pile.take_up_to_n(n)
    }
}
