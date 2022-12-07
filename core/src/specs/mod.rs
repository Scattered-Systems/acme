/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::applications::*;

pub(crate) mod applications;

pub trait Spawnable {
    fn spawn(&mut self) -> scsys::Result<&Self>;
}

#[async_trait::async_trait]
pub trait AsyncSpawable {
    async fn spawn(&mut self) -> scsys::AsyncResult<&Self>;
}
