use derive_more::Constructor;

use super::cards::{CardName, CardNames};

pub enum BuyChoice {
    Buy(CardName),
    None,
}

pub enum PlayChoice {
    Play(CardName),
    None,
}

/** An agent is a thing that decides what to do */
pub trait Agent: std::fmt::Debug {
    fn action_phase(&mut self, playable_cards: &[CardName]) -> PlayChoice;
    fn buy_phase(&mut self, buyable_cards: &[CardName]) -> BuyChoice;
}

#[derive(Debug, Constructor)]
struct BasicPriorities {
    buy_priorities: Vec<CardName>,
    play_priorities: Vec<CardName>,
}
impl Agent for BasicPriorities {
    fn action_phase(&mut self, playable_cards: &[CardName]) -> PlayChoice {
        for p in &self.play_priorities {
            if playable_cards.iter().any(|c| c == p) {
                return PlayChoice::Play(*p);
            }
        }
        PlayChoice::None
    }
    fn buy_phase(&mut self, buyable_cards: &[CardName]) -> BuyChoice {
        for p in &self.buy_priorities {
            if buyable_cards.iter().any(|c| c == p) {
                return BuyChoice::Buy(*p);
            }
        }
        BuyChoice::None
    }
}

pub struct Agents {}
impl Agents {
    #[allow(dead_code)]
    pub fn always_buy_copper() -> impl Agent {
        BasicPriorities::new(vec![CardNames::COPPER], vec![])
    }
    #[allow(dead_code)]
    pub fn greedy_for_duchies() -> impl Agent {
        BasicPriorities::new(
            vec![
                CardNames::DUCHY,
                CardNames::PROVINCE,
                CardNames::SILVER,
                CardNames::COPPER,
            ],
            vec![],
        )
    }
    #[allow(dead_code)]
    pub fn silver_test() -> impl Agent {
        BasicPriorities::new(vec![CardNames::PROVINCE, CardNames::SILVER], vec![])
    }
    #[allow(dead_code)]
    pub fn simple_big_money() -> impl Agent {
        BasicPriorities::new(
            vec![CardNames::PROVINCE, CardNames::GOLD, CardNames::SILVER],
            vec![],
        )
    }
}
