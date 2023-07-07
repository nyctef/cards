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
}

trait Player {}

#[derive(Constructor, Debug)]
struct AlwaysBuyCopper {}

impl Player for AlwaysBuyCopper {}

#[cfg(test)]
mod tests {
    use crate::{AlwaysBuyCopper, Game};

    #[test]
    fn a_game_can_start() {
        let mut game = Game::new();
        let mut player_1 = AlwaysBuyCopper::new();
        game.add_player("Player 1", &mut player_1);
    }
}
