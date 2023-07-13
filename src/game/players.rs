use derive_more::Constructor;

use super::{
    model::{BuyChoice, Card, CardName, CardNames, Cards},
    play_area::PlayArea,
    Game,
};

/** An agent is a thing that decides what to do */
pub trait Agent: std::fmt::Debug {
    fn action_phase(&mut self);
    fn buy_phase(&mut self, buyable_cards: &[CardName]) -> BuyChoice;
}

#[derive(Debug, Constructor)]
struct BuyPriority {
    priorities: Vec<CardName>,
}
impl Agent for BuyPriority {
    fn action_phase(&mut self) {}
    fn buy_phase<'card>(&mut self, buyable_cards: &[CardName]) -> BuyChoice {
        for p in &self.priorities {
            if buyable_cards.iter().any(|c| c == p) {
                return BuyChoice::Buy(*p);
            }
        }
        BuyChoice::None
    }
}

pub struct Agents {}
impl Agents {
    pub fn always_buy_copper() -> impl Agent {
        BuyPriority::new(vec![CardNames::COPPER])
    }
    pub fn greedy_for_duchies() -> impl Agent {
        BuyPriority::new(vec![CardNames::DUCHY, CardNames::SILVER, CardNames::COPPER])
    }
}
