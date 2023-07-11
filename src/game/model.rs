#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct CardName {
    name: &'static str,
}

impl std::fmt::Debug for CardName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{}]", self.name))
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct CardType {
    name: &'static str,
}

pub struct CardTypes {}
impl CardTypes {
    pub const ACTION: CardType = CardType { name: "Action" };
    pub const TREASURE: CardType = CardType { name: "Treasure" };
    pub const VICTORY: CardType = CardType { name: "Victory" };
}

pub struct Card {
    pub name: CardName,
    pub coins_cost: u8,
    types: Vec<CardType>,
}

impl Card {
    pub fn get_name(&self) -> &str {
        self.name.name
    }

    pub fn get_types(&self) -> impl Iterator<Item = CardType> + '_ {
        self.types.iter().cloned()
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)
    }
}

pub struct CardNames {}
impl CardNames {
    pub const COPPER: CardName = CardName { name: "Copper" };
    pub const ESTATE: CardName = CardName { name: "Estate" };
    pub const DUCHY: CardName = CardName { name: "Duchy" };
}

pub struct Cards {}
impl Cards {
    pub fn copper() -> Card {
        Card {
            name: CardNames::COPPER,
            coins_cost: 0,
            types: vec![CardTypes::TREASURE],
        }
    }
    pub fn duchy() -> Card {
        Card {
            name: CardNames::DUCHY,
            coins_cost: 5,
            types: vec![CardTypes::VICTORY],
        }
    }
    pub fn estate() -> Card {
        Card {
            name: CardNames::ESTATE,
            coins_cost: 2,
            types: vec![CardTypes::VICTORY],
        }
    }
}

pub enum BuyChoice {
    Buy(CardName),
    None,
}

pub struct PlayerCounters {
    pub actions: u8,
    pub buys: u8,
    pub coins: u8,
}

impl std::fmt::Debug for PlayerCounters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "[{}A {}B {}C]",
            self.actions, self.buys, self.coins
        ))
    }
}

impl PlayerCounters {
    pub fn new_turn() -> Self {
        PlayerCounters {
            actions: 1,
            buys: 1,
            coins: 0,
        }
    }
}
