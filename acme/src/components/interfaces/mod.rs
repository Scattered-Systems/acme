/*
    Appellation: interfaces <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use interface::*;

mod interface;

pub mod flavors {
    use async_trait::async_trait;

    /// Describe a standard api specification to implement on compatible structures
    #[async_trait]
    pub trait APISpec<App> {
        async fn client() -> Result<Self, scsys::BoxError>
            where
                Self: Sized;
        async fn router() -> Result<axum::Router, scsys::BoxError>
            where
                Self: Sized;
        async fn app(&self) -> Result<App, scsys::BoxError>
            where
                Self: Sized;
    }

    /// Describe a standard cli specification to implement on compatible structures
    pub trait CLISpec<App: clap::Parser> {
        fn app(&self) -> Result<(), scsys::BoxError>
            where
                Self: Sized;
    }
}
