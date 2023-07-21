use std::collections::HashMap;

use derive_more::Constructor;

use super::{
    cards::{CardName, CardNames},
    effects::CardEffect,
};

pub enum BuyChoice {
    Buy(CardName),
    None,
}

pub enum PlayChoice {
    Play(CardName),
    None,
}

/** An agent is a thing that decides what to do */
pub trait Agent: std::fmt::Debug {
    fn action_phase(&mut self, playable_cards: &[CardName]) -> PlayChoice;
    fn buy_phase(&mut self, buyable_cards: &[CardName]) -> BuyChoice;
}

trait Agent2 {
    fn request_choice(&mut self, choice: &ChoiceRequest) -> ChoiceResponse;
}

struct ChoiceResponse(Box<[CardName]>);

// eg Library might have a stack like this:
//  Phase(ActionPhase) > Card(Library) > Effect(DrawCards(7)) > Question(DiscardQ(<card>))
// to ask the agent to maybe discard a card and keep drawing

// TODO: what about other choices? like "gain +1$ or +1 buy"
// or Moat deciding whether to reveal your card or not (although there's not really any reason not to?)
// those can probably just be extra values in a ChoiceResponse enum
// I guess the main downside is that the request/responses are less well-typed?
// would need to handle the cases where agents give invalid responses more explicitly.
// or maybe just a few top-level methods like request_card_choice and request_effect_choice

struct ChoiceRequest(Box<[TurnStateStackEntry]>);

enum TurnStateStackEntry {
    Phase(TurnPhase),
    Card(CardName),
    Effect(CardEffect),
    Question(AgentQuestion),
}

enum TurnPhase {
    Buy,
    Action,
    Cleanup,
    Night,
    // some way of marking this, but another player's turn (in case of attacks?)
    /* ... */
}

enum AgentQuestion {
    DiscardQ(CardName),
}

#[derive(Debug, Constructor)]
struct BasicPriorities {
    buy_priorities: Vec<CardName>,
    play_priorities: Vec<CardName>,
}
impl Agent for BasicPriorities {
    fn action_phase(&mut self, playable_cards: &[CardName]) -> PlayChoice {
        for p in &self.play_priorities {
            if playable_cards.iter().any(|c| c == p) {
                return PlayChoice::Play(*p);
            }
        }
        PlayChoice::None
    }
    fn buy_phase(&mut self, buyable_cards: &[CardName]) -> BuyChoice {
        for p in &self.buy_priorities {
            if buyable_cards.iter().any(|c| c == p) {
                return BuyChoice::Buy(*p);
            }
        }
        BuyChoice::None
    }
}

#[derive(Debug)]
struct BoundedPriorities {
    buy_priorities: Vec<(CardName, u8)>,
    play_priorities: Vec<CardName>,
    purchased_cards: HashMap<CardName, u8>,
}
impl BoundedPriorities {
    fn new(buy_priorities: Vec<(CardName, u8)>, play_priorities: Vec<CardName>) -> Self {
        Self {
            buy_priorities,
            play_priorities,
            purchased_cards: HashMap::<CardName, u8>::new(),
        }
    }
}
impl Agent for BoundedPriorities {
    fn action_phase(&mut self, playable_cards: &[CardName]) -> PlayChoice {
        for p in &self.play_priorities {
            if playable_cards.iter().any(|c| c == p) {
                return PlayChoice::Play(*p);
            }
        }
        PlayChoice::None
    }

    fn buy_phase(&mut self, buyable_cards: &[CardName]) -> BuyChoice {
        for (candidate, max_we_want) in &self.buy_priorities {
            let already_gained_count = self.purchased_cards.entry(*candidate).or_insert(0);
            if *already_gained_count >= *max_we_want {
                continue;
            }
            if buyable_cards.iter().any(|c| c == candidate) {
                *already_gained_count += 1;
                return BuyChoice::Buy(*candidate);
            }
        }
        BuyChoice::None
    }
}

pub struct Agents {}
impl Agents {
    #[allow(dead_code)]
    pub fn always_buy_copper() -> impl Agent {
        BasicPriorities::new(vec![CardNames::COPPER], vec![])
    }
    #[allow(dead_code)]
    pub fn greedy_for_duchies() -> impl Agent {
        BasicPriorities::new(
            vec![
                CardNames::DUCHY,
                CardNames::PROVINCE,
                CardNames::SILVER,
                CardNames::COPPER,
            ],
            vec![],
        )
    }
    #[allow(dead_code)]
    pub fn silver_test() -> impl Agent {
        BasicPriorities::new(vec![CardNames::PROVINCE, CardNames::SILVER], vec![])
    }
    #[allow(dead_code)]
    pub fn simple_big_money() -> impl Agent {
        BasicPriorities::new(
            vec![CardNames::PROVINCE, CardNames::GOLD, CardNames::SILVER],
            vec![],
        )
    }
    #[allow(dead_code)]
    pub fn big_money_splash_smithys() -> impl Agent {
        BoundedPriorities::new(
            vec![
                (CardNames::PROVINCE, 100),
                (CardNames::SMITHY, 2),
                // (CardNames::VILLAGE, 2),
                (CardNames::GOLD, 100),
                (CardNames::SILVER, 100),
            ],
            vec![CardNames::VILLAGE, CardNames::SMITHY],
        )
    }
}
