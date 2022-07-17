/*
    Appellation: interface <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Interface {
    pub hash: String,
    pub id: u64,
    pub key: String,

    pub mode: String,
    pub name: String,
    pub operations: Vec<String>,
    pub secret: String,
    pub timestamp: i64,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize, y: usize| x + y;
        assert_eq!(f(4, 2), 6)
    }
}
