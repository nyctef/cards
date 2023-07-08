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

/** A player is an entity that holds some cards. An Agent decides what actions the Player takes
 * TODO: should Player just be replaced with PlayArea?
 */
#[derive(Debug)]
pub struct Player {
    play_area: PlayArea<CopperToken>,
}

impl Player {
    pub fn new() -> Self {
        Player {
            play_area: PlayArea::new(),
        }
    }
    pub fn give_initial_cards(&mut self, copper_count: u8) {
        self.play_area = PlayArea::from_initial_cards(vec![CopperToken {}; copper_count.into()]);
    }

    pub fn cleanup(&mut self) {
        self.play_area.discard_hand();
        self.play_area.draw_hand();
    }

    pub fn gain_card_to_discard_pile(&mut self, card: CopperToken) {
        self.play_area.gain_card_to_discard_pile(card);
    }

    #[cfg(test)]
    pub fn debug_copper_count(&self) -> u8 {
        self.play_area.debug_total_card_count()
    }
}

#[derive(Debug, Constructor)]
pub struct AlwaysBuyCopper {}

impl Agent for AlwaysBuyCopper {
    fn action_phase(&mut self) {}
    fn buy_phase(&mut self) -> BuyChoice {
        BuyChoice::Buy(CopperToken {})
    }
}
