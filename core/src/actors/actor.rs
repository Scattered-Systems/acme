/*
   Appellation: actor
   Context: module
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ...Summary...
*/

type CBuilderDS = config::ConfigBuilder<config::builder::DefaultState>;


pub trait ActorSpec<Addr, Cnf, Cnt, Dt> {
    fn authenticate(&self, address: Addr) -> bool where Self: Sized;
    fn configure(&self, configuration: Cnf) -> Result<Self, config::ConfigError> where Self: Sized;
    fn connect(&self, endpoint: Addr) -> Self where Self: Sized;
    fn determine(&self, context: Cnt, data: Dt) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized;
}