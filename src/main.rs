use game::{players::Agents, shuffler::RandomShuffler, Game};
use logs::NullLog;

mod game;
mod logs;

fn main() {
    let log = NullLog::new();
    let shuffler = RandomShuffler::unseeded();
    let mut game = Game::new(&log);
    let mut player_1 = Agents::greedy_for_duchies();
    let mut player_2 = Agents::always_buy_copper();
    game.add_player("P1 [GFD]", &mut player_1, &shuffler);
    game.add_player("P2 [ABC]", &mut player_2, &shuffler);
    game.populate_basic_kingdom();

    let results = game.play_to_end();
    println!("{}", results);
}
