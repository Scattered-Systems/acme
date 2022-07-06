/*
   Appellation: interface
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/

use tower_http::{
    compression::CompressionLayer, propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveHeadersLayer, trace,
};

pub type AxumServer =
axum::Server<hyper::server::conn::AddrIncoming, axum::routing::IntoMakeService<axum::Router>>;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Interface {
    pub address: std::net::SocketAddr,
    pub context: crate::Context,
}

impl Interface {
    pub async fn new() -> Self {
        let configuration = crate::Configuration::new().ok().unwrap();

        crate::Logger::setup(&configuration);

        let host = [0, 0, 0, 0];
        let port = configuration.server.port;

        let address: std::net::SocketAddr = std::net::SocketAddr::from((host, port));
        let context = crate::Context::new(configuration.clone());

        let client = axum::Router::new()
            .merge(crate::api::endpoints::index::create_route())
            .layer(
                trace::TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().include_headers(true))
                    .on_request(trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
            )
            .layer(SetSensitiveHeadersLayer::new(std::iter::once(
                http::header::AUTHORIZATION,
            )))
            .layer(CompressionLayer::new())
            .layer(PropagateHeaderLayer::new(
                http::header::HeaderName::from_static("x-request-id"),
            ))
            .layer(axum::Extension(context.clone()));

        println!("{}", &configuration.server);

        axum::Server::bind(&address)
            .serve(client.into_make_service())
            .await
            .expect("Server Error");

        Self { address, context }
    }
}

impl std::fmt::Display for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Interface()", )
    }
}
