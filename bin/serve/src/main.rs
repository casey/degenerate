use {
  axum::{http::StatusCode, routing::get_service, Router},
  std::net::SocketAddr,
  tower_http::{services::ServeDir, trace::TraceLayer},
  tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt},
};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

async fn serve() -> Result {
  tracing_subscriber::registry()
    .with(tracing_subscriber::EnvFilter::from_default_env())
    .with(tracing_subscriber::fmt::layer())
    .init();

  let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
  eprintln!("Listening on {}", addr);

  let app = Router::new()
    .fallback(
      get_service(ServeDir::new("www")).handle_error(|err| async move {
        (
          StatusCode::INTERNAL_SERVER_ERROR,
          format!("I/O error: {}", err),
        )
      }),
    )
    .layer(TraceLayer::new_for_http());

  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await?;

  Ok(())
}

#[tokio::main]
async fn main() -> Result {
  serve().await?;

  Ok(())
}
