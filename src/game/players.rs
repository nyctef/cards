use derive_more::Constructor;

pub trait Player: std::fmt::Debug {
    fn action_phase(&mut self);
    fn buy_phase(&mut self, copper_count: &mut u8);
    fn cleanup(&mut self);
}

#[derive(Constructor, Debug)]
pub struct AlwaysBuyCopper {}

impl Player for AlwaysBuyCopper {
    fn action_phase(&mut self) {}

    fn buy_phase(&mut self, copper_count: &mut u8) {
        *copper_count -= 1;
    }

    fn cleanup(&mut self) {}
}
