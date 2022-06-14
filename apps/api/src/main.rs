pub mod application;
pub mod context;
pub mod endpoints;
pub mod logger;
pub mod settings;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    application::Application::setup().run().await?;
    Ok(())
}