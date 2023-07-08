#![allow(unused)]

mod deck;
mod play_area;
mod players;

use derive_more::Constructor;

use self::{
    play_area::PlayArea,
    players::{Agent, AlwaysBuyCopper, BuyChoice, CopperToken},
};
use crate::logs::{GameEvent, GameLog};

#[derive(Debug)]
struct Game<'a> {
    players: Vec<(&'a str, PlayArea<CopperToken>, &'a mut dyn Agent)>,
    // temporary
    copper_count: u8,
    log: &'a dyn GameLog,
}
impl<'a> Game<'a> {
    fn new(log: &'a dyn GameLog) -> Self {
        Self {
            players: Vec::new(),
            copper_count: 0,
            log,
        }
    }

    fn add_player(&mut self, name: &'a str, agent: &'a mut dyn Agent) {
        let player = PlayArea::new();
        self.players.push((name, player, agent));
    }

    fn play_one_turn(&mut self) {
        for (name, area, agent) in self.players.iter_mut() {
            let action_choice = agent.action_phase();
            let buy_choice = agent.buy_phase();
            match buy_choice {
                BuyChoice::Buy(CopperToken { .. }) => {
                    self.copper_count -= 1;
                    area.gain_card_to_discard_pile(CopperToken {});
                    self.log
                        .record(GameEvent::Todo(format!("{} gained 1 copper", name)));
                }
                BuyChoice::None => {}
            }
            area.discard_hand();
            area.draw_hand(self.log);
        }
    }

    fn populate_basic_kingdom(&mut self) {
        self.copper_count = 60;
    }

    fn deal_starting_hands(&mut self) {
        for (name, area, agent) in self.players.iter_mut() {
            area.gain_cards_to_discard_pile(&mut vec![CopperToken {}; 7]);
            self.copper_count -= 7;

            area.draw_hand(self.log);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logs::tests::TestLog;
    use std::cell::RefCell;

    #[test]
    fn a_game_can_start_and_a_player_can_buy_something() {
        let log = TestLog::new();
        let mut game = Game::new(&log);
        let mut player_1 = AlwaysBuyCopper::new();
        game.add_player("Player 1", &mut player_1);
        game.populate_basic_kingdom();
        game.deal_starting_hands();
        game.play_one_turn();

        insta::assert_snapshot!(log.dump())
    }
}
