pub struct PlayerCounters {
    pub actions: u8,
    pub buys: u8,
    pub coins: u8,
}

impl std::fmt::Debug for PlayerCounters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "[{}A {}B {}C]",
            self.actions, self.buys, self.coins
        ))
    }
}

impl PlayerCounters {
    pub fn new_turn() -> Self {
        PlayerCounters {
            actions: 1,
            buys: 1,
            coins: 0,
        }
    }
}
