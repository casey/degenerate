use {
  super::*,
  axum::{http::StatusCode, response::IntoResponse, routing::get_service, Router},
  chromiumoxide::browser::BrowserConfig,
  futures::StreamExt,
  std::{io, net::SocketAddr, process::Command, sync::Once, time::Duration},
  tokio::task,
  tower_http::{services::ServeDir, trace::TraceLayer},
  tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt},
};

struct Browser {
  browser_handle: task::JoinHandle<()>,
  inner: chromiumoxide::Browser,
  port: u16,
  server_handle: task::JoinHandle<()>,
}

impl Browser {
  async fn new() -> Result<Self> {
    let (inner, mut handler) = chromiumoxide::Browser::launch(
      BrowserConfig::builder()
        .arg("--allow-insecure-localhost")
        .build()?,
    )
    .await?;

    let addr = SocketAddr::from(([127, 0, 0, 1], 0));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let port = listener.local_addr()?.port();
    drop(listener);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::trace!("Listening on {}", addr);

    let app = Router::new()
      .fallback(get_service(ServeDir::new("tests/www")).handle_error(Browser::handle_error))
      .layer(TraceLayer::new_for_http());

    let server = axum::Server::bind(&addr).serve(app.into_make_service());

    let server_handle = task::spawn(async move { server.await.unwrap() });

    let browser_handle = task::spawn(async move {
      loop {
        let _ = handler.next().await.unwrap();
      }
    });

    Ok(Browser {
      browser_handle,
      inner,
      port,
      server_handle,
    })
  }

  async fn handle_error(err: io::Error) -> impl IntoResponse {
    (
      StatusCode::INTERNAL_SERVER_ERROR,
      format!("I/O error: {}", err),
    )
  }
}

impl Drop for Browser {
  fn drop(&mut self) {
    self.browser_handle.abort();
    self.server_handle.abort();
  }
}

fn setup() {
  static ONCE: Once = Once::new();

  ONCE.call_once(|| {
    tracing_subscriber::registry()
      .with(tracing_subscriber::EnvFilter::from_default_env())
      .with(tracing_subscriber::fmt::layer())
      .init();

    eprintln!("Building WASM binary...");

    let status = Command::new("cargo")
      .args(["build", "--release", "--target", "wasm32-unknown-unknown"])
      .status()
      .unwrap();

    if !status.success() {
      panic!("Failed to build WASM binary: {status}");
    }

    eprintln!("Running wasm-bindgen...");

    let status = Command::new("wasm-bindgen")
      .args([
        "--target",
        "web",
        "--no-typescript",
        "target/wasm32-unknown-unknown/release/degenerate.wasm",
        "--out-dir",
        "tests/www",
      ])
      .status()
      .unwrap();

    if !status.success() {
      panic!("wasm-bindgen failed: {status}");
    }

    eprintln!("Done with setup!");
  });
}

pub async fn test(name: &str, program: &str) -> Result {
  super::clean();

  setup();

  eprintln!("Launching browser...");

  let browser = Browser::new().await?;

  eprintln!("Creating page...");

  let page = browser
    .inner
    .new_page(format!("http://127.0.0.1:{}", browser.port))
    .await?;

  eprintln!("Waiting for module to load...");

  loop {
    if page
      .evaluate(format!("window.test"))
      .await?
      .value()
      .is_some()
    {
      break;
    }

    tokio::time::sleep(Duration::from_millis(100)).await;
  }

  eprintln!("Running test...");

  let data_url = page
    .evaluate(format!("window.test('{program}')"))
    .await?
    .into_value::<String>()?;

  let have = image::load_from_memory(&base64::decode(
    &data_url["data:image/png;base64,".len()..],
  )?)?;

  let want_path = format!("images/{}.png", name);

  let want = image::open(&want_path)?;

  if have != want {
    let destination = format!("images/{}.browser-actual-memory.png", name);

    have.save(&destination)?;

    set_label_red(&destination)?;

    panic!(
      "Image test failed:\nExpected: {}\nActual:   {}",
      want_path, destination,
    );
  }

  Ok(())
}
