/*
   Appellation: acme
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use primitives::*;

mod primitives;

#[doc(inline)]
#[cfg(feature = "default")]
pub use acme_sdk::*;
#[doc(inline)]
#[cfg(feature = "full")]
pub use acme_sdk::*;


#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}
