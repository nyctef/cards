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

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::cell::RefCell;

    #[derive(Debug)]
    pub struct TestLog {
        messages: RefCell<Vec<String>>,
    }
    impl TestLog {
        pub fn new() -> Self {
            TestLog {
                messages: vec![].into(),
            }
        }

        pub fn dump(&self) -> String {
            self.messages.borrow().join("\n")
        }
    }
    impl GameLog for TestLog {
        fn record(&self, event: GameEvent) {
            self.messages.borrow_mut().push(format!("{:?}", event))
        }
    }
}
