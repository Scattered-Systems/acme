/*
   Appellation: errors
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use std::error::Error;

type BoxedDynErrorSSs = Box<dyn Error + Send + Sync + 'static>;

pub enum ChainErrors {
    Configuration(config::ConfigError),
    Dynamic(BoxedDynErrorSSs),
}
