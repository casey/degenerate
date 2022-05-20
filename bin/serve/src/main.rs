use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result {
  tracing_subscriber::registry()
    .with(tracing_subscriber::EnvFilter::from_default_env())
    .with(tracing_subscriber::fmt::layer())
    .init();

  serve::run(8000).await?;

  Ok(())
}
