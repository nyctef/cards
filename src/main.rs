use derive_more::Constructor;

fn main() {
    println!("Hello, world!");
}
#[derive(Constructor, Debug)]
struct Game {}
impl Game {
    fn add_player(&mut self, name: &str, player: &mut dyn Player) {
        todo!()
    }

    fn play_one_turn(&self) {
        todo!()
    }

    fn debug_copper_supply_count(&self) -> u8 {
        todo!()
    }

    fn populate_basic_kingdom(&self) {
        todo!()
    }

    fn deal_starting_hands(&self) {
        todo!()
    }
}

trait Player {}

#[derive(Constructor, Debug)]
struct AlwaysBuyCopper {}

impl Player for AlwaysBuyCopper {}

#[cfg(test)]
mod tests {
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
        assert_eq!(53, game.debug_copper_supply_count(), "7 coppers dealt to one player");
        game.play_one_turn();
        assert_eq!(52, game.debug_copper_supply_count(), "player should have bought one copper");
    }
}
