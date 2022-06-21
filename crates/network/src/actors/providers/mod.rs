/*
    Appellation: Providers
    Context: Module
    Creator: Joe McCain III <jo3mccain@gmail.com> (https://pzzld.eth.link/)
    Description:

 */
mod provider;
pub use provider::*;

pub trait ProviderSpec {
    type Actor;
    type Configuration;
    type Context;
    type Data;

    fn authenticate(&self) -> Self;
    fn configure(&self, context: Self::Context) -> Self::Context;
    fn constructor(actor: Self::Actor) -> Self;
}

