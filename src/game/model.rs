#[derive(PartialEq, Eq, Clone, Copy)]
pub struct CardName {
    name: &'static str,
}

impl std::fmt::Debug for CardName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{}]", self.name))
    }
}

pub struct Card {
    pub name: CardName,
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
        }
    }
    pub fn duchy() -> Card {
        Card {
            name: CardNames::DUCHY,
        }
    }
    pub fn estate() -> Card {
        Card {
            name: CardNames::ESTATE,
        }
    }
}

impl Card {
    pub fn get_name(&self) -> &str {
        self.name.name
    }
}

pub enum BuyChoice {
    Buy(CardName),
    None,
}
