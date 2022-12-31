/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{apps::*, cli::*};

pub(crate) mod apps;
#[cfg(feature = "cli")]
pub(crate) mod cli;

use async_trait::async_trait;

///
#[async_trait]
pub trait AsyncHandler: Clone + Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn handler(&self) -> Result<&Self, Self::Error>
    where
        Self: Sized;
}
///
#[async_trait]
pub trait AsyncSpawable {
    async fn spawn(&mut self) -> scsys::AsyncResult<&Self>;
}
///
pub trait BaseApplication: BaseObject + Versionable {
    fn application(&self) -> &Self {
        self
    }
    fn namespace(&self) -> String;
}
///
pub trait BaseObject {
    fn count(&self) -> usize;
    fn name(&self) -> String;
    fn slug(&self) -> String {
        self.name().to_ascii_lowercase()
    }
    fn symbol(&self) -> String;
}
///
pub trait Handler: Clone {
    type Error: std::error::Error + 'static;

    fn handler(&self) -> Result<&Self, Self::Error>
    where
        Self: Sized;
}
///
pub trait Spawnable {
    fn spawn(&mut self) -> scsys::Result<&Self>;
}
///
pub trait Versionable {
    type Error;

    fn update(&mut self) -> Result<(), Box<Self::Error>>;
    fn version(&self) -> String;
}
