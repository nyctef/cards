#![allow(unused)]

mod deck;
mod play_area;
mod players;

use derive_more::Constructor;

use self::players::{Agent, AlwaysBuyCopper, BuyChoice, CopperToken, Player};

#[derive(Debug)]
struct Game<'a> {
    players: Vec<(Player, &'a mut dyn Agent)>,
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

    fn add_player(&mut self, name: &str, agent: &'a mut dyn Agent) {
        let player = Player::new();
        self.players.push((player, agent));
    }

    fn play_one_turn(&mut self) {
        for (player, agent) in self.players.iter_mut() {
            let action_choice = agent.action_phase();
            let buy_choice = agent.buy_phase();
            match buy_choice {
                BuyChoice::Buy(CopperToken { .. }) => {
                    self.copper_count -= 1;
                    player.gain_card_to_discard_pile(CopperToken {})
                }
                BuyChoice::None => {}
            }
            player.cleanup();
        }
    }

    fn populate_basic_kingdom(&mut self) {
        self.copper_count = 60;
    }

    fn deal_starting_hands(&mut self) {
        for (player, agent) in self.players.iter_mut() {
            player.give_initial_cards(7);
            self.copper_count -= 7;
            // cleanup here is a shorthand for "draw first five cards"
            player.cleanup();
        }
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
        // assert_eq!(0, player_1.debug_copper_count());
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
