use derive_more::Constructor;

use super::{
    model::{BuyChoice, Card, CardName, CardNames, Cards},
    play_area::PlayArea,
    Game,
};

/** An agent is a thing that decides what to do */
pub trait Agent: std::fmt::Debug {
    fn action_phase(&mut self) -> ();
    fn buy_phase<'card>(&mut self, buyable_cards: &Vec<CardName>) -> BuyChoice;
}

#[derive(Debug, Constructor)]
struct BuyPriority {
    priorities: Vec<CardName>,
}
impl Agent for BuyPriority {
    fn action_phase(&mut self) {}
    fn buy_phase<'card>(&mut self, buyable_cards: &Vec<CardName>) -> BuyChoice {
        for p in &self.priorities {
            if buyable_cards.iter().find(|c| *c == p).is_some() {
                return BuyChoice::Buy(*p);
            }
        }
        return BuyChoice::None;
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
