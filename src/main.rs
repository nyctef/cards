use std::{rc::Rc, cmp::Ordering};

use crate::game::{
    logs::{GameLog, NullLog},
    players::Agents,
    shuffler::RandomShuffler,
    Game,
};

mod game;

fn main() {
    let shuffler = RandomShuffler::unseeded();
    let mut p1_wins = 0;
    let mut p2_wins = 0;
    let mut draws = 0;

    for _ in 0..1000 {
        let log = GameLog::new(Rc::new(NullLog::new()));
        let mut game = Game::new(log);
        let mut player_1 = Agents::simple_big_money();
        let mut player_2 = Agents::big_money_splash_smithys();
        game.add_player("P1 [SBM]", &mut player_1, &shuffler);
        game.add_player("P2 [BMS]", &mut player_2, &shuffler);
        game.populate_basic_kingdom();
        game.populate_prosperous_kingdom();
        game.populate_some_actions();

        let results = game.play_to_end();
        // TODO: figure out some nicer way to get at these results
        let p1_score = results.0.get(0).unwrap().score;
        let p2_score = results.0.get(1).unwrap().score;
        match p1_score.cmp(&p2_score) {
            Ordering::Greater => p1_wins += 1,
            Ordering::Equal => draws += 1,
            Ordering::Less => p2_wins += 1,
        }
    }

    dbg!(p1_wins, p2_wins, draws);
}
