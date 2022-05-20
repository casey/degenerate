use {
  axum::{http::StatusCode, routing::get_service, Router},
  std::net::SocketAddr,
  tower_http::{services::ServeDir, services::ServeFile, trace::TraceLayer},
};

pub async fn run(port: u16) -> Result<(), Box<dyn std::error::Error>> {
  let addr = SocketAddr::from(([127, 0, 0, 1], port));
  eprintln!("Listening on {}", addr);

  let app = Router::new()
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
    .layer(TraceLayer::new_for_http());

  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await?;

  Ok(())
}
