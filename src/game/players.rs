use std::collections::HashMap;

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

#[derive(Debug)]
struct BoundedPriorities {
    buy_priorities: Vec<(CardName, u8)>,
    play_priorities: Vec<CardName>,
    purchased_cards: HashMap<CardName, u8>,
}
impl BoundedPriorities {
    fn new(buy_priorities: Vec<(CardName, u8)>, play_priorities: Vec<CardName>) -> Self {
        Self {
            buy_priorities,
            play_priorities,
            purchased_cards: HashMap::<CardName, u8>::new(),
        }
    }
}
impl Agent for BoundedPriorities {
    fn action_phase(&mut self, playable_cards: &[CardName]) -> PlayChoice {
        for p in &self.play_priorities {
            if playable_cards.iter().any(|c| c == p) {
                return PlayChoice::Play(*p);
            }
        }
        PlayChoice::None
    }

    fn buy_phase(&mut self, buyable_cards: &[CardName]) -> BuyChoice {
        for (candidate, max_we_want) in &self.buy_priorities {
            let already_gained_count = self.purchased_cards.entry(*candidate).or_insert(0);
            if *already_gained_count >= *max_we_want {
                continue;
            }
            if buyable_cards.iter().any(|c| c == candidate) {
                *already_gained_count += 1;
                return BuyChoice::Buy(*candidate);
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
    #[allow(dead_code)]
    pub fn big_money_splash_smithys() -> impl Agent {
        BoundedPriorities::new(
            vec![
                (CardNames::PROVINCE, 100),
                (CardNames::SMITHY, 2),
                // (CardNames::VILLAGE, 2),
                (CardNames::GOLD, 100),
                (CardNames::SILVER, 100),
            ],
            vec![CardNames::VILLAGE, CardNames::SMITHY],
        )
    }
}
