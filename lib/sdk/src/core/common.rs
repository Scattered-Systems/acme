/*
    Appellation: common
    Context: module
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
 */

mod constants {}

mod types {
    use bson;
    use chrono;
    use std::collections::HashMap;

    pub type LocalTime = chrono::Local;
    pub type Dict<T = Vec<String>> = HashMap<String, T>;

    pub enum Ids {
        Alien(String),
        Objects(bson::oid::ObjectId),
        Standard(u64),
    }
}

mod variants {}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}