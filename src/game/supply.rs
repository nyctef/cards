use super::{
    card_pile::CardPile,
    cards::{Card, CardName},
};

#[derive(Debug)]
pub struct Supply {
    supply_piles: Vec<NamedCardPile>,
}

struct NamedCardPile {
    name: CardName,
    pile: CardPile,
}

impl std::fmt::Debug for NamedCardPile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.pile.fmt(f)
    }
}

impl Supply {
    pub fn new() -> Self {
        Supply {
            supply_piles: vec![],
        }
    }

    /** We remember the top card the pile originally had,
     * so we have a name to use once it's empty
     */
    pub fn empty_supply_piles(&self) -> impl Iterator<Item = CardName> + '_ {
        self.supply_piles
            .iter()
            .filter(|s| s.pile.is_empty())
            .map(|s| s.name)
    }

    pub fn buyable_cards(&self, coins: u8) -> impl Iterator<Item = CardName> + '_ {
        self.supply_piles.iter().filter_map(move |s| {
            s.pile
                .peek()
                .filter(|c| c.coins_cost <= coins)
                .map(|c| c.name)
        })
    }

    /** Note that this looks up piles by the current top card
     * (matching what's returned from `buyable_cards`)
     * instead of using the original pile name.
     */
    fn supply_pile_for(&mut self, card: CardName) -> Option<&mut CardPile> {
        self.supply_piles
            .iter_mut()
            .find(|s| s.pile.peek().map(|c| c.name) == Some(card))
            .map(|p| &mut p.pile)
    }

    pub fn take_one(&mut self, card: CardName) -> Option<Card> {
        self.take_up_to_n(card, 1).into_iter().next()
    }

    pub fn add(&mut self, vec: Vec<Card>) {
        let pile: CardPile = vec.into();
        // TODO: is this still useful enough when we start having mixed supply piles?
        let name = pile.peek().expect("Can't add empty pile").name;
        self.supply_piles.push(NamedCardPile { name, pile });
    }

    pub fn take_up_to_n(&mut self, card: CardName, n: usize) -> Vec<Card> {
        let pile = self
            .supply_pile_for(card)
            .expect(format!("Missing supply pile for {:?}", card).as_str());
        pile.take_up_to_n(n)
    }

    pub fn clear(&mut self) {
        // TODO: some way to reuse the vecs here too?
        self.supply_piles = vec![];
    }
}
