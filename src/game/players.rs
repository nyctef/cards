use derive_more::Constructor;

pub trait Player: std::fmt::Debug {
    fn give_initial_cards(&mut self, copper_count: u8);
    fn action_phase(&mut self);
    fn buy_phase(&mut self, copper_count: &mut u8);
    fn cleanup(&mut self);
}

#[derive(Debug)]
pub struct AlwaysBuyCopper {
    my_copper_count: u8,
}

impl Player for AlwaysBuyCopper {
    fn give_initial_cards(&mut self, copper_count: u8) {
        self.my_copper_count += copper_count;
    }

    fn action_phase(&mut self) {}

    fn buy_phase(&mut self, copper_count: &mut u8) {
        *copper_count -= 1;
        self.my_copper_count += 1;
    }

    fn cleanup(&mut self) {}
}

impl AlwaysBuyCopper {
    pub fn new() -> Self {
        AlwaysBuyCopper { my_copper_count: 0 }
    }

    #[cfg(test)]
    pub fn debug_copper_count(&self) -> u8 {
        self.my_copper_count
    }
}
