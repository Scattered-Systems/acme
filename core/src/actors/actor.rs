/*
    Appellation: actor
    Context: module
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */

pub trait ActorSpec {
    type Actor;
    type Client;
    type Configuration;
    type Data;

    fn authenticate(&self, client: Self::Client) -> Self::Client;
    fn configure(&self, configuration: Self::Configuration) -> Self::Configuration;
    fn constructor(&self) -> Self::Actor;
    fn datastream(&self, destination: String) -> Self::Data;
}