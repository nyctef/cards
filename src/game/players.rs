use derive_more::Constructor;

use super::{play_area::PlayArea, Game};

pub enum BuyChoice {
    Buy(CopperToken),
    None,
}

/** An agent is a thing that decides what to do */
pub trait Agent: std::fmt::Debug {
    fn action_phase(&mut self) -> ();
    fn buy_phase(&mut self) -> BuyChoice;
}

#[derive(Debug, Clone)]
pub struct CopperToken {}

#[derive(Debug, Constructor)]
pub struct AlwaysBuyCopper {}

impl Agent for AlwaysBuyCopper {
    fn action_phase(&mut self) {}
    fn buy_phase(&mut self) -> BuyChoice {
        BuyChoice::Buy(CopperToken {})
    }
}
