use super::model::{Card, CardName};

#[derive(Debug)]
pub struct Supply {
    supply_piles: Vec<Vec<Card>>,
}

impl Supply {
    pub fn new() -> Self {
        Supply {
            supply_piles: vec![],
        }
    }

    pub fn buyable_cards(&self) -> Vec<CardName> {
        self.supply_piles
            .iter()
            .filter_map(|s| s.iter().last().map(|c| c.name))
            .collect()
    }

    pub fn supply_pile_for(&mut self, card: CardName) -> Option<&mut Vec<Card>> {
        self.supply_piles
            .iter_mut()
            .filter(|s| s.last().map(|c| c.name) == Some(card))
            .next()
    }

    pub fn take_from_supply(&mut self, card: CardName) -> Option<Card> {
        self.supply_pile_for(card).and_then(|p| p.pop())
    }

    pub fn add(&mut self, vec: Vec<Card>) {
        self.supply_piles.push(vec);
    }
}
