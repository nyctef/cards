use derive_more::Constructor;

use super::play_area::PlayArea;

pub trait Player: std::fmt::Debug {
    fn give_initial_cards(&mut self, copper_count: u8);
    fn action_phase(&mut self);
    fn buy_phase(&mut self, copper_count: &mut u8);
    fn cleanup(&mut self);
}

#[derive(Debug, Clone)]
struct CopperToken {}

#[derive(Debug)]
pub struct AlwaysBuyCopper {
    play_area: PlayArea<CopperToken>,
}

impl AlwaysBuyCopper {
    pub fn new() -> Self {
        AlwaysBuyCopper {
            play_area: PlayArea::new(),
        }
    }

    #[cfg(test)]
    pub fn debug_copper_count(&self) -> u8 {
        self.play_area.debug_total_card_count()
    }
}

impl Player for AlwaysBuyCopper {
    fn give_initial_cards(&mut self, copper_count: u8) {
        self.play_area = PlayArea::from_initial_cards(vec![CopperToken {}; copper_count.into()]);
    }

    fn action_phase(&mut self) {}

    fn buy_phase(&mut self, copper_count: &mut u8) {
        *copper_count -= 1;
        self.play_area.gain_card_to_discard_pile(CopperToken {})
    }

    fn cleanup(&mut self) {
        self.play_area.discard_hand();
        self.play_area.draw_hand();
    }
}
