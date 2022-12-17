/*
    Appellation: middleware <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{gateway::*, specs::*};

pub(crate) mod gateway;

pub(crate) mod specs {
    use async_trait::async_trait;
    use scsys::AsyncResult;
    use tower::layer::Layer;

    #[async_trait]
    pub trait GatewayService {
        fn handle(&self) -> String;
        fn service(&self) -> &Self {
            self
        }
        async fn start(&mut self) -> AsyncResult<&Self>;
    }

    pub trait GatewayServiceLayer<S: GatewayService>: Layer<S> {}

    pub trait IntoGatewayService {}
}
