#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardEffect {
    None,
    Sequence(Box<[CardEffect]>),
    AddActions(u8),
    // AddBuys(u8),
    AddCoins(u8),
    DrawCards(u8),
    // TrashCardsFromHand(TrashInstruction),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrashInstruction {
    pub min_cards_to_trash: u8,
    pub max_cards_to_trash: u8,
}
