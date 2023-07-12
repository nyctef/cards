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
pub struct AlwaysBuyCopper {}

impl Agent for AlwaysBuyCopper {
    fn action_phase(&mut self) {}
    fn buy_phase<'card>(&mut self, buyable_cards: &Vec<CardName>) -> BuyChoice {
        buyable_cards
            .iter()
            .find(|c| **c == CardNames::COPPER)
            .map(|c| BuyChoice::Buy(*c))
            .unwrap_or(BuyChoice::None)
    }
}

#[derive(Debug, Constructor)]
pub struct GreedyForDuchies {}

impl Agent for GreedyForDuchies {
    fn action_phase(&mut self) {}
    fn buy_phase<'card>(&mut self, buyable_cards: &Vec<CardName>) -> BuyChoice {
        let priorities = [CardNames::DUCHY, CardNames::COPPER];
        for p in priorities {
            if buyable_cards.iter().find(|c| **c == p).is_some() {
                return BuyChoice::Buy(p);
            }
        }
        return BuyChoice::None;
    }
}
