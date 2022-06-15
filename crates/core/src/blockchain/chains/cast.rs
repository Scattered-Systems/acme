use crate::types::BoxedError;

pub trait ChainSpec {
    type Configuration;

    fn setup() -> Self;
    fn connect() -> Result<(), BoxedError>;
}