use std::fmt;

#[derive(Debug)]
pub struct Error {
    kind: Kind,
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        "An eventsourcing error ocurred"
    }

    fn cause(&self) -> Option<&std::error::Error> {
        None
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            Kind::ApplicationFailure(ref s) => fmt::Display::fmt(s, f),
            Kind::CommandFailure(ref s) => fmt::Display::fmt(s, f),
            Kind::StoreFailure(ref s) => fmt::Display::fmt(s, f),
        }
    }
}

/// Indicates the kind of event sourcing error that occurred.
#[derive(Debug)]
pub enum Kind {
    ApplicationFailure(String),
    CommandFailure(String),
    StoreFailure(String),
}

/// A Result where failure is an event sourcing error
pub type Result<T> = std::result::Result<T, Error>;

pub trait Event {
    fn event_type(&self) -> &str;
}

pub trait Aggregate {
    type Event: Event;
    type Command;
    type State: AggregateState;

    fn apply_event(state: &Self::State, evt: Self::Event) -> Result<Self::State>;
    fn handle_command(state: &Self::State, cmd: Self::Command) -> Result<Vec<Self::Event>>;
}

pub trait AggregateState {
    fn generation(&self) -> u64;
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
