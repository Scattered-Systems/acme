/*
    Appellation: common
    Context: module
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
 */
pub use constants::*;
pub use types::*;
pub use variants::*;

mod constants {}

mod types {
    pub type LocalTime = chrono::Local;
    pub type Dict<T = Vec<String>> = std::collections::HashMap<String, T>;
}

mod variants {
    pub enum Ids {
        Alien(String),
        Objects(bson::oid::ObjectId),
        Standard(u64),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}