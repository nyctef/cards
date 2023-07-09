// TODO: Card equality: do we want the option of just finding some card
// with the same name vs actually trying to track specific instances of
// a card with an ID or something?
#[derive(Debug, PartialEq, Eq)]
pub struct Card<'a> {
    name: &'a str,
}

pub struct Cards {}
impl Cards {
    pub fn copper() -> Card<'static> {
        Card { name: "Copper" }
    }
}

impl Card<'_> {
    pub fn get_name(&self) -> &str {
        self.name
    }
}

pub enum BuyChoice<'a> {
    Buy(&'a Card<'a>),
    None,
}
