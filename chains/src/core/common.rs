/*
   Appellation: common
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use constants::*;
pub use types::*;

mod constants {
    pub const DIFFICULTY_PREFIX: &str = "00";
}

mod types {
    pub type BData = String;
    pub type BId = u64;
    pub type BHash = String;
    pub type BNonce = u64;
    pub type BTStamp = i64;
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}
