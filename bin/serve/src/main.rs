use {
  axum::{
    http::{
      header::{self, HeaderValue},
      StatusCode,
    },
    routing::get_service,
    Router,
  },
  std::{
    net::ToSocketAddrs,
    process::{self, Command},
  },
  tower_http::{
    services::ServeDir, services::ServeFile, set_header::SetResponseHeaderLayer, trace::TraceLayer,
  },
  tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  tracing_subscriber::registry()
    .with(tracing_subscriber::EnvFilter::from_default_env())
    .with(tracing_subscriber::fmt::layer())
    .init();

  let mut watch = Command::new("cargo")
    .arg("watch")
    .arg("--shell")
    .arg(concat!(
      "cargo build --release --target wasm32-unknown-unknown",
      " && ",
      "wasm-bindgen --target web --no-typescript target/wasm32-unknown-unknown/release/degenerate.wasm --out-dir www",
      " && ",
      "cargo build --release --target wasm32-unknown-unknown --package program",
      " && ",
      "wasm-bindgen --target web --no-typescript target/wasm32-unknown-unknown/release/program.wasm --out-dir www"
    ))
    .spawn()?;

  ctrlc::set_handler(move || {
    watch.kill().unwrap();
    process::exit(0);
  })
  .expect("Error setting Ctrl-C handler");

  let addr = ("0.0.0.0", 80)
    .to_socket_addrs()?
    .next()
    .ok_or_else(|| format!("failed to get socket addrs"))?;

  let router = Router::new()
    .fallback(
      get_service(ServeDir::new("www").fallback(ServeFile::new("www/index.html"))).handle_error(
        |err| async move {
          (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("I/O error: {}", err),
          )
        },
      ),
    )
    .layer(SetResponseHeaderLayer::overriding(
      header::CACHE_CONTROL,
      HeaderValue::from_static("no-store"),
    ))
    .layer(TraceLayer::new_for_http());

  axum::Server::bind(&addr)
    .serve(router.into_make_service())
    .await?;

  Ok(())
}
