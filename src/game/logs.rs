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
    // TODO: a more generic way of storing key-value pairs in data
    // like Attributes in the tracing crate
    fn enter_span<'a>(&self, span_name: &'static str, data: &'a str) -> SpanId;
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
    pub fn enter_turn<'a>(&self, player_name: &'a str) -> GameLogSpan {
        GameLogSpan::new(
            self.inner.enter_span("turn", player_name),
            self.inner.clone(),
        )
    }
    pub fn enter_cleanup(&self) -> GameLogSpan {
        GameLogSpan::new(self.inner.enter_span("cleanup", ""), self.inner.clone())
    }

    pub fn enter_buy_phase(&self) -> GameLogSpan {
        GameLogSpan::new(self.inner.enter_span("buy phase", ""), self.inner.clone())
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
        self.print(format!("{:?}", event));
    }

    fn enter_span<'a>(&self, name: &'static str, data: &'a str) -> SpanId {
        self.print(format!("{}: {}", name, data));
        *self.indent.borrow_mut() += 1;
        SpanId::new(self.indent.borrow().clone())
    }

    fn exit_span(&self, id: SpanId) {
        debug_assert!(id == SpanId::new(*self.indent.borrow().deref()));
        *self.indent.borrow_mut() -= 1;
    }
}

impl ConsoleLog {
    fn print(&self, str: String) {
        // TODO: is there a nicer way to build `indent` here?
        let indent = "  ".repeat(TryInto::<usize>::try_into(*(self.indent.borrow())).unwrap());
        println!("{}{}", indent, str)
    }
}

#[derive(Debug, Constructor)]
pub struct NullLog;
impl GameLogInner for NullLog {
    fn record(&self, _event: GameEvent) {}
    fn enter_span<'a>(&self, _name: &'static str, data: &'a str) -> SpanId {
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

        fn print(&self, str: String) {
            // TODO: is there a nicer way to build `indent` here?
            let indent = "  ".repeat(TryInto::<usize>::try_into(*(self.indent.borrow())).unwrap());
            self.messages
                .borrow_mut()
                .push(format!("{}{}", indent, str))
        }
    }
    impl GameLogInner for TestLog {
        fn record(&self, event: GameEvent) {
            self.print(format!("{:?}", event));
        }

        fn enter_span<'a>(&self, name: &'static str, data: &'a str) -> SpanId {
            self.print(format!("{}: {}", name, data));
            *self.indent.borrow_mut() += 1;
            SpanId::new(0)
        }

        fn exit_span(&self, id: SpanId) {
            *self.indent.borrow_mut() -= 1;
        }
    }
}
