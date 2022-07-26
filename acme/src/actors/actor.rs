/*
    Appellation: actor <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

pub trait ActorSpec {
    fn appellation(&self) -> (String, String)
        where
            Self: Sized;
}
