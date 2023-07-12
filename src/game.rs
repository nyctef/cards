#![allow(unused)]

mod card_pile;
mod model;
mod play_area;
mod players;
mod supply;

use std::fmt::{Display, Formatter};

use self::{
    model::{BuyChoice, Card, CardName, CardNames, CardTypes, Cards, PlayerCounters},
    play_area::PlayArea,
    players::{Agent, AlwaysBuyCopper},
    supply::Supply,
};
use crate::logs::{GameEvent, GameLog};
use derive_more::Constructor;
use itertools::Itertools;

#[derive(Debug)]
struct Game<'a> {
    players: Vec<(&'a str, PlayArea, &'a mut dyn Agent)>,
    supply: Supply,
    log: &'a dyn GameLog,
    max_turns: u8,
}
impl<'a> Game<'a> {
    fn new(log: &'a dyn GameLog) -> Self {
        Self {
            players: vec![],
            supply: Supply::new(),
            log,
            max_turns: 100,
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
                .filter(|c| c.get_types().find(|t| *t == CardTypes::TREASURE).is_some())
                .map(|c| c.name)
                .collect_vec()
            {
                area.play_card(c, &mut player_counters);
                self.log.record(GameEvent::Todo(format!(
                    "{:?} {} played a {:?}",
                    player_counters, name, c
                )));
            }

            let buyable_cards = self.supply.buyable_cards(player_counters.coins);
            let buy_choice = agent.buy_phase(&buyable_cards.collect());
            match buy_choice {
                BuyChoice::Buy(card) => {
                    let purchased = self.supply.take_one(card).expect("TODO");
                    area.gain_card_to_discard_pile(purchased);
                    self.log
                        .record(GameEvent::Todo(format!("{} gained a {:?}", name, card)));
                }
                BuyChoice::None => {}
            }
            area.discard_in_play();
            area.discard_hand();
            area.draw_hand(self.log);
        }
    }

    fn populate_basic_kingdom(&mut self) {
        self.populate_supply(|| Cards::copper(), 60);
        self.populate_supply(|| Cards::silver(), 40);
        self.populate_supply(|| Cards::gold(), 30);
        self.populate_supply(|| Cards::estate(), 12);
        self.populate_supply(|| Cards::duchy(), 12);
    }

    fn populate_supply(&mut self, printer: impl Fn() -> Card, count: u8) {
        self.supply
            .add((0..count).into_iter().map(|_| printer()).collect());
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

    fn has_ended(&mut self) -> bool {
        // TODO
        self.max_turns -= 1;
        self.max_turns <= 0
            || self
                .supply
                .empty_supply_piles()
                // TODO: check for type == victory rather than just by name
                // TODO: check for 3/4 empty supply piles
                .any(|s| s == CardNames::DUCHY)
    }

    fn collect_cards_and_get_results(&mut self) -> PlayerResults {
        let mut results = vec![];
        // we could totally do this in a nondestructive way with references to
        // player cards rather than actually moving the card objects around,
        // but this way seems more fun
        for (name, area, _) in self.players.iter_mut() {
            let mut player_cards = area.take_all_cards();
            player_cards.sort_by_key(|c| c.name);
            // TODO
            // let score = self.calculate_score(&player_cards);
            let score = 0;
            results.push(PlayerResult::new(name, player_cards, score));
        }
        PlayerResults { 0: results }
    }
}

#[derive(Debug, Constructor)]
struct PlayerResult<'a> {
    name: &'a str,
    cards: Vec<Card>,
    score: u8,
}

impl Display for PlayerResult<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}: {} points", self.name, self.score)?;
        for (name, cards) in &self.cards.iter().group_by(|c| c.name) {
            writeln!(f, "  {:?} x{}", name, cards.count())?
        }
        Ok(())
    }
}

// create a newtype since we can't directly impl Display for Vec
// https://github.com/apolitical/impl-display-for-vec
struct PlayerResults<'a>(pub Vec<PlayerResult<'a>>);
impl Display for PlayerResults<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for result in &self.0 {
            writeln!(f, "{}", result)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{game::players::GreedyForDuchies, logs::tests::TestLog};
    use std::cell::RefCell;

    #[test]
    fn a_game_can_start_and_a_player_can_buy_something() {
        let log = TestLog::new();
        let mut game = Game::new(&log);
        let mut player_1 = AlwaysBuyCopper::new();
        game.add_player("Player 1", &mut player_1);
        game.populate_supply(|| Cards::copper(), 10);
        game.populate_supply(|| Cards::estate(), 3);
        game.deal_starting_hands();
        game.play_one_turn();

        insta::assert_snapshot!(log.dump());
        insta::assert_debug_snapshot!((game.players, game.supply));
    }

    #[test]
    fn can_buy_duchies_with_a_cheap_strategy() {
        let log = TestLog::new();
        let mut game = Game::new(&log);
        let mut player_1 = GreedyForDuchies::new();
        game.add_player("Player 1", &mut player_1);
        game.populate_supply(|| Cards::copper(), 10);
        game.populate_supply(|| Cards::estate(), 3);
        game.populate_supply(|| Cards::duchy(), 3);
        game.deal_starting_hands();
        for t in 0..5 {
            game.play_one_turn();
        }

        insta::assert_snapshot!(log.dump());
        insta::assert_debug_snapshot!((game.players, game.supply));
    }

    #[test]
    fn one_player_beats_another_buy_eventually_buying_enough_duchies() {
        // TODO: make sure the game ends via supply running out rather than turn limit
        // TODO: introduce (seeded) randomness by shuffling the player's decks
        // TODO: print number of turns (per player) in results
        // TODO: print the game end reason to the log
        let log = TestLog::new();
        let mut game = Game::new(&log);
        let mut player_1 = GreedyForDuchies::new();
        let mut player_2 = AlwaysBuyCopper::new();
        game.add_player("P1 [GFD]", &mut player_1);
        game.add_player("P2 [ABC]", &mut player_2);
        game.populate_basic_kingdom();
        game.deal_starting_hands();
        while !game.has_ended() {
            game.play_one_turn();
        }
        let results = game.collect_cards_and_get_results();

        insta::assert_snapshot!(log.dump());
        insta::assert_display_snapshot!(results);
    }
}
