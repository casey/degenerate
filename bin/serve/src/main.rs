use {
  std::net::SocketAddr,
  tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt},
};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result {
  tracing_subscriber::registry()
    .with(tracing_subscriber::EnvFilter::from_default_env())
    .with(tracing_subscriber::fmt::layer())
    .init();

  serve::run(SocketAddr::from(([127, 0, 0, 1], 8000))).await?;

  Ok(())
}
