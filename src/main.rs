#![allow(unused)]

use derive_more::Constructor;

fn main() {
    println!("Hello, world!");
}
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

trait Player: std::fmt::Debug {}

#[derive(Constructor, Debug)]
struct AlwaysBuyCopper {}

impl Player for AlwaysBuyCopper {}

#[cfg(test)]
mod game_tests {
    use crate::{AlwaysBuyCopper, Game};

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

struct TurnState {
    money: u8,
}

impl TurnState {
    fn new() -> Self {
        Self { money: 0 }
    }

    #[cfg(test)]
    fn debug_money(&self) -> u8 {
        self.money
    }
}

trait Card {
    fn play(&self, turn_state: &mut TurnState);
}

struct Cards {
    
}
impl Cards {
    fn copper() -> impl Card {
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
        turn_state.money += self.money;
    }
}

#[cfg(test)]
mod card_tests {
    use crate::{TurnState, Cards, Card};

    #[test]
    fn playing_a_copper_gives_1_money() {
        let mut turn_state = TurnState::new();
        assert_eq!(0, turn_state.debug_money());

        let copper = Cards::copper();
        copper.play(&mut turn_state);
        assert_eq!(1, turn_state.debug_money());
    }
}
