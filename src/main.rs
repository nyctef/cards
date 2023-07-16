use std::rc::Rc;

use game::{
    logs::{ConsoleLog, GameLog},
    players::Agents,
    shuffler::RandomShuffler,
    Game,
};

mod game;

fn main() {
    let log = GameLog::new(Rc::new(ConsoleLog::new()));
    let shuffler = RandomShuffler::unseeded();
    let mut game = Game::new(log);
    let mut player_1 = Agents::simple_big_money();
    let mut player_2 = Agents::silver_test();
    game.add_player("P1 [SBM]", &mut player_1, &shuffler);
    game.add_player("P2 [ST]", &mut player_2, &shuffler);
    game.populate_basic_kingdom();
    game.populate_prosperous_kingdom();
    game.populate_some_actions();

    let results = game.play_to_end();
    println!("{}", results);
}
