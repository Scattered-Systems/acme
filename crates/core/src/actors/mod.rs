/*
    Appellation: Actors
    Context: module
    Creator: FL03 <jo3mccain@icloud.com>
    Description:

 */

pub trait Actionable {
    type Action;
    type Config;
    type Context;
    type Data;

    fn constructor(action: Self::Action, config: Self::Config, data: Self::Data) -> Self;
    fn determine(
        &self
    ) -> Result<
        Self,
        Box<dyn std::error::Error + Send + Sync + 'static>
    >
        where Self: Sized;
}