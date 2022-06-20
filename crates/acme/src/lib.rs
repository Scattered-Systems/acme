pub use bson::oid::ObjectId;

#[doc(inline)]
#[cfg(feature = "data")]
pub use acme_data::*;
#[doc(inline)]
#[cfg(feature = "macros")]
pub use acme_macros::*;
#[doc(inline)]
#[cfg(feature = "network")]
pub use acme_network::*;
pub use actors::*;
pub use common::*;
pub use controllers::*;
pub use utilities::*;

/*
    Library: acme-core
    Version: 0.1.5

    Overview

 */
pub mod application;
mod actors;
mod controllers;
mod utilities;

mod common {
    use bson;
    use chrono;

    pub enum Dates {
        Datetime(chrono::DateTime<chrono::Local>),
        Localtime(chrono::Local),
        Timestamp(bson::DateTime),
    }

    pub type DateTime = chrono::DateTime<LocalTime>;
    pub type LocalTime = chrono::Local;
    pub type TimeStamp = bson::DateTime;
}

pub mod errors {
    use std::error::Error;

    pub use config::ConfigError;

    pub enum Errors {
        Default(BoxedError)
    }

    pub type AsyncError = Box<dyn Error + Send + Sync + 'static>;
    pub type BoxedError = Box<dyn Error>;
}

