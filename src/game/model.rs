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
    #[allow(dead_code)] // TODO start implementing some actions
    pub const ACTION: CardType = CardType { name: "Action" };
    pub const TREASURE: CardType = CardType { name: "Treasure" };
    pub const VICTORY: CardType = CardType { name: "Victory" };
}

pub struct Card {
    pub name: CardName,
    pub coins_cost: u8,
    pub treasure_value: u8,
    pub vp_value: u8,
    types: Vec<CardType>,
}

impl Card {
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
    pub const SILVER: CardName = CardName { name: "Silver" };
    pub const GOLD: CardName = CardName { name: "Gold" };
    pub const ESTATE: CardName = CardName { name: "Estate" };
    pub const DUCHY: CardName = CardName { name: "Duchy" };
}

pub struct Cards {}
impl Cards {
    fn basic_treasure(coins_cost: u8, treasure_value: u8, name: CardName) -> Card {
        Card {
            name,
            coins_cost,
            treasure_value,
            vp_value: 0,
            types: vec![CardTypes::TREASURE],
        }
    }

    fn basic_victory(coins_cost: u8, vp_value: u8, name: CardName) -> Card {
        Card {
            name,
            coins_cost,
            treasure_value: 0,
            vp_value,
            types: vec![CardTypes::VICTORY],
        }
    }

    pub fn copper() -> Card {
        Self::basic_treasure(0, 1, CardNames::COPPER)
    }
    pub fn silver() -> Card {
        Self::basic_treasure(3, 2, CardNames::SILVER)
    }
    pub fn gold() -> Card {
        Self::basic_treasure(6, 3, CardNames::GOLD)
    }
    pub fn estate() -> Card {
        Self::basic_victory(2, 1, CardNames::ESTATE)
    }
    pub fn duchy() -> Card {
        Self::basic_victory(5, 3, CardNames::DUCHY)
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
