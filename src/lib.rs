/*
   Appellation: acme
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
#[doc(inline)]
pub use crate::{common::*, utils::*};

#[doc(inline)]
#[cfg(feature = "default")]
pub use acme_sdk::*;
#[doc(inline)]
#[cfg(feature = "full")]
pub use acme_sdk::*;

mod common {
    pub use types::*;

    mod types {
        pub type ConfigBuilderDS = config::ConfigBuilder<config::builder::DefaultState>;
        pub type ConfigFromFileVec = Vec<config::File<config::FileSourceFile, config::FileFormat>>;
    }
}

mod utils {
    pub fn collect_config_files(pattern: &str) -> crate::ConfigFromFileVec {
        glob::glob(pattern)
            .unwrap()
            .map(|path| config::File::from(path.unwrap()).required(false))
            .collect::<Vec<_>>()
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
