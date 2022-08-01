/*
    Appellation: mod <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

use crate::{async_trait, AxumServer};

/// Describe a standard api specification to implement on compatible structures
#[async_trait]
pub trait APISpec<App = AxumServer> {
    async fn run(&self) -> Result<(), scsys::BoxError>
        where
            Self: Sized + Sync;
}
