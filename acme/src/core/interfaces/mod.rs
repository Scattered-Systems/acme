/*
    Appellation: interfaces <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use flavors::*;
pub use interface::*;

mod interface;

mod flavors {
    use async_trait::async_trait;

    #[async_trait]
    pub trait API {
        async fn client() -> Result<Self, crate::AsyncError>
            where
                Self: Sized;
        async fn router() -> Result<axum::Router, crate::AsyncError>
            where
                Self: Sized;
    }

    pub trait CLI
        where
            Self: clap::Parser,
    {
        fn configure(&self) -> Result<crate::DefaultConfigBuilder, config::ConfigError>
            where
                Self: Sized;
        fn constructor(&self) -> Result<Self, crate::StandardError>
            where
                Self: Sized;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize, y: usize| x + y;
        assert_eq!(f(4, 2), 6)
    }
}
