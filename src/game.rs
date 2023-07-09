#![allow(unused)]

mod card_pile;
mod model;
mod play_area;
mod players;
mod supply;

use self::{
    model::{BuyChoice, Card, CardName, CardNames, Cards, PlayerCounters},
    play_area::PlayArea,
    players::{Agent, AlwaysBuyCopper},
    supply::Supply,
};
use crate::logs::{GameEvent, GameLog};
use derive_more::Constructor;

#[derive(Debug)]
struct Game<'a> {
    players: Vec<(&'a str, PlayArea, &'a mut dyn Agent)>,
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
            let mut player_counters = PlayerCounters::new_turn();
            let action_choice = agent.action_phase();

            for c in area
                .inspect_hand()
                .into_iter()
                .filter(|c| *c == CardNames::COPPER)
            {
                area.play_card(c);
                player_counters.coins += 1;
                self.log.record(GameEvent::Todo(format!(
                    "{:?} {} played a copper",
                    player_counters, name
                )));
            }

            let buyable_cards = self.supply.buyable_cards();
            let buy_choice = agent.buy_phase(&buyable_cards);
            match buy_choice {
                BuyChoice::Buy(card) => {
                    let purchased_copper = self.supply.take_one(card).expect("TODO");
                    area.gain_card_to_discard_pile(purchased_copper);
                    self.log
                        .record(GameEvent::Todo(format!("{} gained 1 copper", name)));
                }
                BuyChoice::None => {}
            }
            area.discard_in_play();
            area.discard_hand();
            area.draw_hand(self.log);
        }
    }

    fn populate_basic_kingdom(&mut self) {
        // TODO technically this should be 60
        // need more test-specific builders

        self.supply
            .add((0..10).into_iter().map(|_| Cards::copper()).collect());
        self.supply
            .add((0..10).into_iter().map(|_| Cards::duchy()).collect());
        self.supply
            .add((0..10).into_iter().map(|_| Cards::estate()).collect());
    }

    fn deal_starting_hands(&mut self) {
        for (name, area, agent) in self.players.iter_mut() {
            let mut coppers = self.supply.take_up_to_n(CardNames::COPPER, 7);
            area.gain_cards_to_discard_pile(&mut coppers);
            let mut estates = self.supply.take_up_to_n(CardNames::ESTATE, 3);
            area.gain_cards_to_discard_pile(&mut estates);

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
