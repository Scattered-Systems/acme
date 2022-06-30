/*
   Appellation: cli
   Context: module
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/

pub trait CommandLineInterface {
    type Application;
    type Config;
    type Context;
    type Data;

    fn client(&self) -> Self::Data;
}

pub trait CLI {
    type Commands;

    fn call(&self) -> Self::Commands;
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}
