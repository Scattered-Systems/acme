use config::ConfigError;

pub trait ConfiguratorSpec {
    type Actor;
    type Context;
    type Container;
    type Data;

    fn constructor(pattern: String) -> Result<Self, ConfigError> where Self: Sized;
}

pub trait ControllerSpec {
    type Actor;
    type Context;
}