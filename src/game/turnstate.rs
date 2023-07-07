pub struct TurnState {
    money: u8,
}

impl TurnState {
    pub fn new() -> Self {
        Self { money: 0 }
    }

    #[cfg(test)]
    pub fn debug_money(&self) -> u8 {
        self.money
    }

    pub fn add_money(&mut self, money: u8) {
       self.money += money; 
    }
}
