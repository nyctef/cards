use derive_more::Constructor;
use std::{cell::RefCell, ops::Deref, rc::Rc};

use self::span_details_are_private::*;
use super::{model::CardName, player_counters::PlayerCounters};

#[derive(Debug, PartialEq, Eq)]
pub enum GameEvent {
    CardPlayed(CardName, PlayerCounters),
    CardBoughtGained(CardName),
    DrawCards(usize),
    Shuffle(),
}

pub trait GameLogInner {
    fn record(&self, event: GameEvent);
    fn enter_span(&self, name: &'static str) -> SpanId;
    fn exit_span(&self, id: SpanId);
}

pub struct GameLog {
    inner: Rc<dyn GameLogInner>,
}
impl GameLog {
    pub fn new(inner: Rc<dyn GameLogInner>) -> Self {
        GameLog { inner: inner }
    }
    pub fn record(&self, event: GameEvent) {
        self.inner.record(event)
    }
    pub fn enter_turn(&self, name: &'static str) -> GameLogSpan {
        GameLogSpan::new(self.inner.enter_span(name), self.inner.clone())
    }
}

impl std::fmt::Debug for GameLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("GameLog"))
    }
}

/** I think this is what https://github.com/rust-lang/rust/issues/34537 wants us to do? */
mod span_details_are_private {
    use derive_more::Constructor;
    use std::rc::Rc;

    use super::GameLogInner;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Constructor)]
    pub struct SpanId(u64);

    #[derive(Constructor)]
    pub struct GameLogSpan {
        id: SpanId,
        log: Rc<dyn GameLogInner>,
    }

    impl std::fmt::Debug for GameLogSpan {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("Span({})", self.id.0))
        }
    }

    impl Drop for GameLogSpan {
        fn drop(&mut self) {
            self.log.exit_span(self.id)
        }
    }
}

#[derive(Debug)]
struct ConsoleLog {
    indent: RefCell<u64>,
}
impl GameLogInner for ConsoleLog {
    fn record(&self, event: GameEvent) {
        // TODO: is there a nicer way to build `indent` here?
        let indent = "  ".repeat(TryInto::<usize>::try_into(*(self.indent.borrow())).unwrap());
        println!("{}{:?}", indent, event)
    }

    fn enter_span(&self, name: &'static str) -> SpanId {
        *self.indent.borrow_mut() += 1;
        SpanId::new(self.indent.borrow().clone())
    }

    fn exit_span(&self, id: SpanId) {
        debug_assert!(id == SpanId::new(*self.indent.borrow().deref()));
        *self.indent.borrow_mut() -= 1;
    }
}

#[derive(Debug, Constructor)]
pub struct NullLog;
impl GameLogInner for NullLog {
    fn record(&self, _event: GameEvent) {}
    fn enter_span(&self, _name: &'static str) -> SpanId {
        SpanId::new(0)
    }
    fn exit_span(&self, _id: SpanId) {}
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::cell::RefCell;

    #[derive(Debug)]
    pub struct TestLog {
        messages: RefCell<Vec<String>>,
        indent: RefCell<u64>,
    }
    impl TestLog {
        pub fn new() -> Self {
            TestLog {
                messages: vec![].into(),
                indent: 0.into(),
            }
        }

        pub fn dump(&self) -> String {
            self.messages.borrow().join("\n")
        }

        fn print(&self, event: impl std::fmt::Debug) {
            let indent = "  ".repeat(TryInto::<usize>::try_into(*(self.indent.borrow())).unwrap());
            self.messages
                .borrow_mut()
                .push(format!("{}{:?}", indent, event))
        }
    }
    impl GameLogInner for TestLog {
        fn record(&self, event: GameEvent) {
            // TODO: is there a nicer way to build `indent` here?
            self.print(event);
        }

        fn enter_span(&self, name: &'static str) -> SpanId {
            self.print(format!("{}", name));
            *self.indent.borrow_mut() += 1;
            SpanId::new(0)
        }

        fn exit_span(&self, id: SpanId) {
            *self.indent.borrow_mut() -= 1;
        }
    }
}
