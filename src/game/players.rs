use derive_more::Constructor;

pub trait Player: std::fmt::Debug {}

#[derive(Constructor, Debug)]
pub struct AlwaysBuyCopper {}

impl Player for AlwaysBuyCopper {}
