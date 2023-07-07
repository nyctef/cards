use derive_more::Constructor;

use super::turnstate::TurnState;

pub trait Card {
    fn play(&self, turn_state: &mut TurnState);
}

pub struct Cards {}
impl Cards {
    pub fn copper() -> impl Card {
        BasicTreasure::new(1, "Copper")
    }
}

#[derive(Debug, Constructor)]
struct BasicTreasure {
    money: u8,
    // TODO: is it worth trying to give this a proper lifetime?
    name: &'static str,
}

impl Card for BasicTreasure {
    fn play(&self, turn_state: &mut TurnState) {
        turn_state.add_money(self.money);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn playing_a_copper_gives_1_money() {
        let mut turn_state = TurnState::new();
        assert_eq!(0, turn_state.debug_money());

        let copper = Cards::copper();
        copper.play(&mut turn_state);
        assert_eq!(1, turn_state.debug_money());
    }
}
