#![allow(unused)]
#![allow(clippy::expect_fun_call)]

mod card_pile;
mod model;
mod play_area;
mod players;
mod shuffler;
mod supply;

use std::fmt::{Display, Formatter};

use self::{
    model::{BuyChoice, Card, CardName, CardNames, CardTypes, Cards, PlayerCounters},
    play_area::PlayArea,
    players::Agent,
    shuffler::Shuffler,
    supply::Supply,
};
use crate::logs::{GameEvent, GameLog};
use derive_more::Constructor;
use itertools::Itertools;

#[derive(Debug)]
struct Game<'a> {
    players: Vec<(&'a str, PlayArea<'a>, &'a mut dyn Agent)>,
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

    fn add_player(
        &mut self,
        name: &'a str,
        agent: &'a mut dyn Agent,
        shuffler: &'a dyn Shuffler<Card>,
    ) {
        let area = PlayArea::new(shuffler);
        self.players.push((name, area, agent));
    }

    fn play_one_turn(&mut self) {
        self.max_turns -= 1;
        for (name, area, agent) in self.players.iter_mut() {
            let mut player_counters = PlayerCounters::new_turn();
            // TODO: implement actions
            agent.action_phase();

            for c in area
                .inspect_hand()
                .filter(|c| c.get_types().any(|t| t == CardTypes::TREASURE))
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
            let buy_choice = agent.buy_phase(&buyable_cards.collect_vec());
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

            if Self::has_ended(self.max_turns, &self.supply) {
                return;
            }

            area.draw_hand(self.log);
        }
    }

    fn populate_basic_kingdom(&mut self) {
        self.populate_supply(Cards::copper, 60);
        self.populate_supply(Cards::silver, 40);
        self.populate_supply(Cards::gold, 30);
        self.populate_supply(Cards::estate, 12);
        self.populate_supply(Cards::duchy, 12);
    }

    fn populate_supply(&mut self, printer: impl Fn() -> Card, count: u8) {
        self.supply.add((0..count).map(|_| printer()).collect());
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

    /** Annoyingly we can't just take &self here, since we want to call this from another method:
     * https://stackoverflow.com/a/32405737
     */
    fn has_ended(max_turns: u8, supply: &Supply) -> bool {
        max_turns == 0
            || supply
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
            let score = Self::calculate_score(&player_cards);
            results.push(PlayerResult::new(name, player_cards, score));
        }
        PlayerResults(results)
    }

    fn calculate_score(player_cards: &[Card]) -> u8 {
        player_cards.iter().map(|c| c.vp_value).sum()
    }

    fn play_to_end(&mut self) -> PlayerResults {
        self.deal_starting_hands();

        while !Self::has_ended(self.max_turns, &self.supply) {
            self.play_one_turn();
        }
        self.collect_cards_and_get_results()
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
    use crate::{
        game::{
            players::Agents,
            shuffler::{NoShuffle, RandomShuffler},
        },
        logs::tests::TestLog,
    };
    use std::cell::RefCell;

    #[test]
    fn a_game_can_start_and_a_player_can_buy_something() {
        let log = TestLog::new();
        let mut shuffler = NoShuffle::new();
        let mut game = Game::new(&log);
        let mut player_1 = Agents::always_buy_copper();
        game.add_player("Player 1", &mut player_1, &shuffler);
        game.populate_supply(Cards::copper, 10);
        game.populate_supply(Cards::estate, 3);
        game.deal_starting_hands();
        game.play_one_turn();

        insta::assert_snapshot!(log.dump());
        insta::assert_debug_snapshot!((game.players, game.supply));
    }

    #[test]
    fn can_buy_duchies_with_a_cheap_strategy() {
        let log = TestLog::new();
        let mut shuffler = NoShuffle::new();
        let mut game = Game::new(&log);
        let mut player_1 = Agents::greedy_for_duchies();
        game.add_player("Player 1", &mut player_1, &shuffler);
        game.populate_supply(Cards::copper, 10);
        game.populate_supply(Cards::estate, 3);
        game.populate_supply(Cards::duchy, 3);
        game.deal_starting_hands();
        for t in 0..5 {
            game.play_one_turn();
        }

        insta::assert_snapshot!(log.dump());
        insta::assert_debug_snapshot!((game.players, game.supply));
    }

    #[test]
    fn one_player_beats_another_buy_eventually_buying_enough_duchies() {
        // TODO: print number of turns (per player) in results
        // TODO: print the game end reason to the log
        let log = TestLog::new();
        // let shuffler = RandomShuffler::new(1234);
        let shuffler = NoShuffle::new();
        let mut game = Game::new(&log);
        let mut player_1 = Agents::greedy_for_duchies();
        let mut player_2 = Agents::always_buy_copper();
        game.add_player("P1 [GFD]", &mut player_1, &shuffler);
        game.add_player("P2 [ABC]", &mut player_2, &shuffler);
        game.populate_basic_kingdom();

        let results = game.play_to_end();

        insta::assert_snapshot!(log.dump());
        insta::assert_display_snapshot!(results);
    }
}
