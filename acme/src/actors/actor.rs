/*
    Appellation: actor
    Context: module
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */

pub trait Actor<A, C> {
    type Appellation;
    type Container;
    type Data;

    fn application(&self) -> A;
    fn constructor(configuration: C) -> Self;
    fn describe(data: Self::Data) -> A;
}