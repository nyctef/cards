#[derive(PartialEq, Eq, Clone, Copy)]
pub struct CardName {
    name: &'static str,
}

impl std::fmt::Debug for CardName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{}]", self.name))
    }
}

impl From<&'static str> for CardName {
    fn from(value: &'static str) -> Self {
        CardName { name: value }
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
    pub fn copper() -> CardName {
        "Copper".into()
    }
}

pub struct Cards {}
impl Cards {
    pub fn copper() -> Card {
        Card {
            name: CardNames::copper(),
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
