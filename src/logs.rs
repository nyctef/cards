#[derive(Debug, PartialEq, Eq)]
pub enum GameEvent {
    Todo(String),
}

pub trait GameLog: std::fmt::Debug {
    fn record(&self, event: GameEvent);
}

#[derive(Debug)]
struct ConsoleLog {}
impl GameLog for ConsoleLog {
    fn record(&self, event: GameEvent) {
        println!("{:?}", event)
    }
}
