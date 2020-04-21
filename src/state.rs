use std::fmt;
use std::hash::Hash;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum Event {
    Idle,
    Error,
    AcceptToken,
    Tokenize,
}

impl fmt::Display for Event {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Event::Idle => write!(formatter, "{}", "idle"),
            Event::Error => write!(formatter, "{}", "error"),
            Event::AcceptToken => write!(formatter, "{}", "accept_token"),
            Event::Tokenize => write!(formatter, "{}", "tokenize"),
        }
    }
}

pub struct State {
    value: String,
    on: HashMap<Event, State>
}

impl State {
    pub fn new(value: &str) -> State {
        let on: HashMap<Event, State> = HashMap::new();

        State {
            on,
            value: String::from(value)
        }
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }
}

impl fmt::Display for State {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.value)
    }
}

impl fmt::Debug for State {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("State")
            .field("value", &self.value)
            .field("on", &self.on)
            .finish()
    }
}

pub struct StateMachine {
    current: State,
    states: HashMap<String, State>
}

impl StateMachine {
    pub fn new(initial: State) -> StateMachine {
        StateMachine {
            current: initial,
            states: HashMap::new()
        }
    }

    pub fn add_transition_listener(&self) {}
    pub fn transition_to(&self, state: State) {}
}
