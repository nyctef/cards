#![allow(unused)]

mod cards;
mod deck;
mod players;
mod turnstate;

use derive_more::Constructor;

use self::{
    players::{AlwaysBuyCopper, Player},
    turnstate::TurnState,
};

#[derive(Debug)]
struct Game<'a> {
    players: Vec<&'a mut dyn Player>,
    // temporary
    copper_count: u8,
}
impl<'a> Game<'a> {
    fn new() -> Self {
        Self {
            players: Vec::new(),
            copper_count: 0,
        }
    }

    fn add_player(&mut self, name: &str, player: &'a mut dyn Player) {
        self.players.push(player);
    }

    fn play_one_turn(&mut self) {
        self.copper_count -= 1;
    }

    fn populate_basic_kingdom(&mut self) {
        self.copper_count = 60;
    }

    fn deal_starting_hands(&mut self) {
        self.copper_count -= 7;
    }

    #[cfg(test)]
    fn debug_copper_supply_count(&self) -> u8 {
        self.copper_count
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn a_game_can_start_and_a_player_can_buy_something() {
        let mut game = Game::new();
        let mut player_1 = AlwaysBuyCopper::new();
        game.add_player("Player 1", &mut player_1);
        assert_eq!(0, game.debug_copper_supply_count());
        game.populate_basic_kingdom();
        assert_eq!(60, game.debug_copper_supply_count());
        game.deal_starting_hands();
        assert_eq!(
            53,
            game.debug_copper_supply_count(),
            "7 coppers dealt to one player"
        );
        game.play_one_turn();
        assert_eq!(
            52,
            game.debug_copper_supply_count(),
            "player should have bought one copper"
        );
    }
}
