use {
  super::*,
  axum::{http::StatusCode, response::IntoResponse, routing::get_service, Router},
  chromiumoxide::browser::{Browser, BrowserConfig},
  futures::StreamExt,
  lazy_static::lazy_static,
  std::{io, net::SocketAddr, process::Command, time::Duration},
  tokio::{runtime::Runtime, task},
  tower_http::{services::ServeDir, trace::TraceLayer},
  tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt},
};

async fn handle_error(err: io::Error) -> impl IntoResponse {
  (
    StatusCode::INTERNAL_SERVER_ERROR,
    format!("I/O error: {}", err),
  )
}

lazy_static! {
  static ref BROWSER: Browser = {
    let runtime: &'static Runtime = &*RUNTIME;

    let (browser, mut handler) = runtime.block_on(async {
      Browser::launch(
        BrowserConfig::builder()
          .arg("--allow-insecure-localhost")
          .build()
          .unwrap(),
      )
      .await
      .unwrap()
    });

    runtime.spawn(async move {
      loop {
        let _ = handler.next().await.unwrap();
      }
    });

    browser
  };
  static ref RUNTIME: Runtime = Runtime::new().unwrap();
  static ref SERVER_PORT: u16 = {
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

    let addr = SocketAddr::from(([127, 0, 0, 1], 0));
    let listener = std::net::TcpListener::bind(addr).unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);

    RUNTIME.spawn(async move {
      let addr = SocketAddr::from(([127, 0, 0, 1], port));
      tracing::trace!("Listening on {}", addr);

      let app = Router::new()
        .fallback(get_service(ServeDir::new("tests/www")).handle_error(handle_error))
        .layer(TraceLayer::new_for_http());

      let server = axum::Server::bind(&addr).serve(app.into_make_service());

      task::spawn(async move { server.await.unwrap() });
    });

    port
  };
}

pub(crate) fn test(name: &str, program: &str) -> Result {
  let browser: &'static Browser = &*BROWSER;
  RUNTIME.block_on(async {
    super::clean();

    eprintln!("Creating page...");

    let page = browser
      .new_page(format!("http://127.0.0.1:{}", *SERVER_PORT))
      .await?;

    eprintln!("Waiting for module to load...");

    loop {
      if page.evaluate("window.test").await?.value().is_some() {
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
  })
}
