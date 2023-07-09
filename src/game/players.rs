use derive_more::Constructor;

use super::{
    model::{BuyChoice, Card},
    play_area::PlayArea,
    Game,
};

/** An agent is a thing that decides what to do */
pub trait Agent: std::fmt::Debug {
    fn action_phase(&mut self) -> ();
    fn buy_phase<'card>(&mut self, buyable_cards: &Vec<&'card Card>) -> BuyChoice<'card>;
}

#[derive(Debug, Constructor)]
pub struct AlwaysBuyCopper {}

impl Agent for AlwaysBuyCopper {
    fn action_phase(&mut self) {}
    fn buy_phase<'card>(&mut self, buyable_cards: &Vec<&'card Card>) -> BuyChoice<'card> {
        buyable_cards
            .iter()
            .filter(|c| c.get_name() == "Copper")
            .next()
            .map(|c| BuyChoice::Buy(c))
            .unwrap_or(BuyChoice::None)
    }
}
