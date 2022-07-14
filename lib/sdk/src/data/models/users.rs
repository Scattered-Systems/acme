/*
   Appellation: users
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Users {
    pub ens: String,
    pub id: crate::Ids,
    pub key: String,
    pub username: String,
    pub password: String,
    pub created: i64,
    pub modified: i64,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}
