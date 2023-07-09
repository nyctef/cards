#![allow(unused)]

mod deck;
mod play_area;
mod players;

use self::{
    play_area::PlayArea,
    players::{Agent, AlwaysBuyCopper, BuyChoice, CopperToken},
};
use crate::logs::{GameEvent, GameLog};
use derive_more::Constructor;

#[derive(Debug)]
struct Supply {
    supply_piles: Vec<Vec<CopperToken>>,
}

impl Supply {
    fn new() -> Self {
        Supply {
            supply_piles: vec![],
        }
    }

    fn buyable_cards(&self) -> Vec<&CopperToken> {
        self.supply_piles
            .iter()
            .filter_map(|s| s.iter().last())
            .collect()
    }

    fn supply_pile_for(&mut self, card: &CopperToken) -> Option<&mut Vec<CopperToken>> {
        self.supply_piles
            .iter_mut()
            .filter(|s| s.last() == Some(card))
            .next()
    }

    fn add(&mut self, vec: Vec<CopperToken>) {
        self.supply_piles.push(vec);
    }
}

#[derive(Debug)]
struct Game<'a> {
    players: Vec<(&'a str, PlayArea<CopperToken>, &'a mut dyn Agent)>,
    supply: Supply,
    log: &'a dyn GameLog,
}
impl<'a> Game<'a> {
    fn new(log: &'a dyn GameLog) -> Self {
        Self {
            players: vec![],
            supply: Supply::new(),
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
                    let supply_pile = self.supply.supply_pile_for(&CopperToken {});
                    let supply_pile = match supply_pile {
                        Some(pile) => pile,
                        None => todo!(),
                    };
                    let purchased_copper = supply_pile.pop().unwrap();
                    area.gain_card_to_discard_pile(purchased_copper);
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
        self.supply.add(vec![CopperToken {}; 60])
    }

    fn deal_starting_hands(&mut self) {
        for (name, area, agent) in self.players.iter_mut() {
            let copper_supply = self.supply.supply_pile_for(&CopperToken {}).unwrap();
            // TODO: some extension method here might be useful since we're doing this a lot
            let split_index = copper_supply.len().saturating_sub(7);
            let mut coppers = copper_supply.split_off(split_index);
            area.gain_cards_to_discard_pile(&mut coppers);

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

        insta::assert_snapshot!(log.dump());
        insta::assert_debug_snapshot!((game.players, game.supply));
    }
}
