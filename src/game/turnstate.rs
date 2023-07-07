pub struct TurnState {
    // todo: make private
    pub money: u8,
}

impl TurnState {
    pub fn new() -> Self {
        Self { money: 0 }
    }

    #[cfg(test)]
    pub fn debug_money(&self) -> u8 {
        self.money
    }
}
