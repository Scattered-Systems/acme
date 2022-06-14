use std::net::SocketAddr;

use axum;
use http::header;
use tower_http::{
    compression::CompressionLayer,
    propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveHeadersLayer,
    trace,
};

use crate::{context::Context, endpoints, logger::Logger, settings::Settings};

#[derive(Clone, Debug)]
pub struct Application {
    pub address: SocketAddr,
    pub context: Context,
}

impl Application {
    pub fn setup() -> Self {
        let settings = match Settings::new() {
            Ok(value) => value,
            Err(err) => panic!("ConfigurationError: {:#?}", err)
        };

        Logger::setup(&settings);

        let host = [0, 0, 0, 0];
        let port = settings.server.port;

        let address: SocketAddr = SocketAddr::from((host, port));
        let context = Context::new(settings.clone());
        Self {
            address,
            context,
        }
    }
    pub async fn run(&mut self) -> Result<Self, Box<dyn std::error::Error>> {
        let client = axum::Router::new()
            .merge(endpoints::base::create_route())
            .layer(
                trace::TraceLayer::new_for_http()
                    .make_span_with(
                        trace::DefaultMakeSpan::new().include_headers(true)
                    )
                    .on_request(
                        trace::DefaultOnRequest::new().level(tracing::Level::INFO)
                    )
                    .on_response(
                        trace::DefaultOnResponse::new().level(tracing::Level::INFO)
                    ),
            )
            .layer(
                SetSensitiveHeadersLayer::new(
                    std::iter::once(
                        header::AUTHORIZATION
                    )
                )
            )
            .layer(
                CompressionLayer::new()
            )
            .layer(
                PropagateHeaderLayer::new(
                    header::HeaderName::from_static(
                        "x-request-id"
                    )
                )
            )
            .layer(axum::Extension(self.context.clone()));

        println!("{}", self.context.settings.server);

        axum::Server::bind(&self.address)
            .serve(client.into_make_service())
            .await
            .expect("Failed to start server");

        Ok(
            Self {
                address: self.address.clone(),
                context: self.context.clone(),
            }
        )
    }
}