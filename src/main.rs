use game::{logs::NullLog, players::Agents, shuffler::RandomShuffler, Game};

mod game;

fn main() {
    let log = NullLog::new();
    let shuffler = RandomShuffler::unseeded();
    let mut game = Game::new(&log);
    let mut player_1 = Agents::greedy_for_duchies();
    let mut player_2 = Agents::silver_test();
    game.add_player("P1 [GFD]", &mut player_1, &shuffler);
    game.add_player("P2 [ST]", &mut player_2, &shuffler);
    game.populate_basic_kingdom();

    let results = game.play_to_end();
    println!("{}", results);
}
