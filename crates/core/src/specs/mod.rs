/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::apps::*;

pub(crate) mod apps;



pub trait BaseApplication: BaseObject + Versionable {
    fn application(&self) -> &Self {
        self
    }
    fn namespace(&self) -> String;
}

pub trait Spawnable {
    fn spawn(&mut self) -> scsys::Result<&Self>;
}

#[async_trait::async_trait]
pub trait AsyncSpawable {
    async fn spawn(&mut self) -> scsys::AsyncResult<&Self>;
}

pub trait BaseObject {
    fn count(&self) -> usize;
    fn name(&self) -> String;
    fn slug(&self) -> String {
        self.name().to_ascii_lowercase()
    }
    fn symbol(&self) -> String;
}

pub trait Versionable {
    type Error;

    fn update(&mut self) -> Result<(), Box<Self::Error>>;
    fn version(&self) -> String;
}
