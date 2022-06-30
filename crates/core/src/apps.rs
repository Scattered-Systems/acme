/*
    Appellation: apps
    Context:
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
 */

pub struct AppContext {
    pub name: String,

}

pub trait Interface {
    type Actor;
    type Client;
    type Context;
    type Data;

    fn authenticate(&self, actor: Self::Actor) -> bool;
    fn constructor(&self, data: Self::Data) -> Self;
}


#[cfg(test)]
mod tests {
    #[test]
    fn simple() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}