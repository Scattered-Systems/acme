/*
    Appellation: interfaces
    Context:
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
 */

pub trait Interfacable {
    type Actor;
    type Client;
    type Config;
    type Data;

    fn activate(&self) -> Self;
    fn configure(&self, config: Self::Config) -> Result<Self::Config, config::ConfigError>;
    fn constructor(&self) -> Self;
}

pub struct Client {
    pub context: String,
    pub data: Vec<String>,
}


pub enum Interfaces {
    Application,
    Client,
}


#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}