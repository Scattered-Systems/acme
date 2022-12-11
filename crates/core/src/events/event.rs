/*
   Appellation: event <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use super::{EventSpec, Eventful};
use scsys::prelude::Message;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Events {
    Await(Event),
    Generic(Event),
    Startup(Event),
}

impl Eventful for Events {
    type Event = Event;

    fn event(&self) -> Self::Event {
        self.into()
    }
}

impl Default for Events {
    fn default() -> Self {
        Self::Generic(Default::default())
    }
}

impl std::convert::From<&Events> for Event {
    fn from(data: &Events) -> Event {
        match data.clone() {
            Events::Await(v) => v,
            Events::Generic(v) => v,
            Events::Startup(v) => v,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Event {
    pub message: Message,
}

impl Event {
    pub fn new(message: Option<Message>) -> Self {
        Self {
            message: message.unwrap_or_default(),
        }
    }
}

impl Default for Event {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Eventful for Event {
    type Event = Self;

    fn event(&self) -> Self::Event {
        self.clone()
    }
}

impl EventSpec for Event {
    type Data = serde_json::Value;

    fn message(&self) -> &scsys::prelude::Message<Self::Data> {
        &self.message
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_events_default() {
        let a = Events::default();
        let b = Events::Startup(Default::default());
        assert_ne!(&a, &b);
    }
}
