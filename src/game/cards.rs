use super::effects::CardEffect;

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
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
    pub vp_value: u8,
    pub effect: CardEffect,
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
    pub const PLATINUM: CardName = CardName { name: "Platinum" };

    pub const ESTATE: CardName = CardName { name: "Estate" };
    pub const DUCHY: CardName = CardName { name: "Duchy" };
    pub const PROVINCE: CardName = CardName { name: "Province" };
    pub const COLONY: CardName = CardName { name: "Colony" };

    pub const SMITHY: CardName = CardName { name: "Smithy" };
    pub const VILLAGE: CardName = CardName { name: "Village" };
}

pub struct Cards {}
impl Cards {
    fn basic_treasure(coins_cost: u8, treasure_value: u8, name: CardName) -> Card {
        Card {
            name,
            coins_cost,
            vp_value: 0,
            types: vec![CardTypes::TREASURE],
            effect: CardEffect::AddCoins(treasure_value),
        }
    }

    fn basic_victory(coins_cost: u8, vp_value: u8, name: CardName) -> Card {
        Card {
            name,
            coins_cost,
            vp_value,
            types: vec![CardTypes::VICTORY],
            effect: CardEffect::None,
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
    pub fn platinum() -> Card {
        Self::basic_treasure(9, 5, CardNames::PLATINUM)
    }

    pub fn estate() -> Card {
        Self::basic_victory(2, 1, CardNames::ESTATE)
    }
    pub fn duchy() -> Card {
        Self::basic_victory(5, 3, CardNames::DUCHY)
    }
    pub fn province() -> Card {
        Self::basic_victory(8, 6, CardNames::PROVINCE)
    }
    pub fn colony() -> Card {
        Self::basic_victory(11, 10, CardNames::COLONY)
    }

    pub fn smithy() -> Card {
        Card {
            name: CardNames::SMITHY,
            coins_cost: 4,
            vp_value: 0,
            types: vec![CardTypes::ACTION],
            effect: CardEffect::DrawCards(3),
        }
    }
    pub fn village() -> Card {
        Card {
            name: CardNames::VILLAGE,
            coins_cost: 3,
            vp_value: 0,
            types: vec![CardTypes::ACTION],
            effect: CardEffect::Sequence(Box::new([
                CardEffect::DrawCards(1),
                CardEffect::AddActions(2),
            ])),
        }
    }
}
