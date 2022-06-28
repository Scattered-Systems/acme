/*
    Appellation: block
    Context:
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */


pub trait Contract {
    type Chain;
    type Data;

    fn compute(&self, data: Self::Data) -> Self;
}