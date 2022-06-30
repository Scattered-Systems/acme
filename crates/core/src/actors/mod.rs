/*
   Appellation: Actors
   Context: module
   Creator: FL03 <jo3mccain@icloud.com>
   Description:

*/

pub enum Actions {
    Connect {
        id: String,
        addresses: std::collections::HashSet<String>,
        status: bool,
    },
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum ActionStatus<T = String> {
    Acting(T),
    Completed(T),
    Exited(T),
}

pub trait ActorSpec {
    type Actor;
    type Client;
    type Connection;
    type Data;
}

pub trait Actionable {
    type Action;
    type Config;
    type Context;
    type Data;

    fn constructor(action: Self::Action, config: Self::Config, data: Self::Data) -> Self;
    fn determine(&self) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>>
    where
        Self: Sized;
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_module() {
        let f = |x: usize, y: usize| x + y;
        assert_eq!(f(10, 10), 20)
    }
}
