use {
  axum::{http::StatusCode, response::IntoResponse, routing::get_service, Router},
  chromiumoxide::browser::BrowserConfig,
  chromiumoxide::handler::viewport::Viewport,
  futures::StreamExt,
  image::io::Reader as ImageReader,
  std::io,
  std::{
    net::SocketAddr,
    process::Command,
    sync::Once,
    time::{Duration, Instant},
  },
  tokio::task,
  tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
  },
  tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt},
};

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

static TRACING: Once = Once::new();

async fn handle_error(err: io::Error) -> impl IntoResponse {
  (
    StatusCode::INTERNAL_SERVER_ERROR,
    format!("I/O error: {}", err),
  )
}

struct Browser {
  inner: chromiumoxide::Browser,
  handle: task::JoinHandle<()>,
  port: u16,
}

impl Browser {
  async fn new() -> Result<Self> {
    TRACING.call_once(|| {
      tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
          std::env::var("RUST_LOG").unwrap_or_else(|_| "".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    });

    let (inner, mut handler) = chromiumoxide::Browser::launch(
      BrowserConfig::builder()
        .arg("--allow-insecure-localhost")
        //.with_head()
        .window_size(256, 256)
        .viewport(Viewport {
          width: 256,
          height: 256,
          device_scale_factor: Some(1.0),
          ..Viewport::default()
        })
        .build()?,
    )
    .await?;

    let addr = SocketAddr::from(([127, 0, 0, 1], 0));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let port = listener.local_addr()?.port();
    drop(listener);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::trace!("listening on {}", addr);

    let app = Router::new()
      .fallback(
        get_service(ServeDir::new("www").fallback(ServeFile::new("www/index.html")))
          .handle_error(handle_error),
      )
      .layer(TraceLayer::new_for_http());

    let server = axum::Server::bind(&addr).serve(app.into_make_service());

    task::spawn(async move { server.await });

    let handle = task::spawn(async move {
      loop {
        let _ = handler.next().await.unwrap();
      }
    });

    Ok(Browser {
      port,
      inner,
      handle,
    })
  }
}

impl Drop for Browser {
  fn drop(&mut self) {
    self.handle.abort();
  }
}

pub struct Test {
  filename: String,
  program: String,
}

impl Test {
  fn new() -> Self {
    Self {
      filename: String::new(),
      program: String::new(),
    }
  }

  fn filename(self, filename: impl AsRef<str>) -> Self {
    Self {
      filename: filename.as_ref().to_owned(),
      ..self
    }
  }

  fn program(self, program: impl AsRef<str>) -> Self {
    Self {
      program: program.as_ref().to_owned(),
      ..self
    }
  }

  async fn run(&self) -> Result {
    super::clean();

    eprintln!("Launching browser...");

    let browser = Browser::new().await?;

    eprintln!("creating page...");

    let page = browser
      .inner
      .new_page(format!("http://127.0.0.1:{}", browser.port))
      .await?;
    page.wait_for_navigation().await?;

    eprintln!("Setting program on textarea...");

    page
      .evaluate(format!(
        "document.getElementsByTagName('textarea')[0].value = '{}'",
        self.program
      ))
      .await?;

    let start = Instant::now();

    loop {
      page
        .find_elements("textarea")
        .await?
        .first()
        .ok_or("Could not find textarea")?
        .type_str(" ")
        .await?;

      let done = page.evaluate("window.done").await?.into_value::<bool>()?;

      let errors = page
        .evaluate("window.errors")
        .await?
        .into_value::<Vec<String>>()?;

      if done || !errors.is_empty() {
        break;
      }

      if Instant::now().duration_since(start) > Duration::from_secs(10000) {
        panic!("Test took more than 60 seconds");
      }

      tokio::time::sleep(Duration::from_millis(100)).await;
    }

    let errors = page
      .evaluate("window.errors")
      .await?
      .into_value::<Vec<String>>()?;

    if !errors.is_empty() {
      for error in errors {
        eprintln!("{}", error);
      }

      panic!("Test encountered errors");
    }

    eprintln!("Grabbing data url from canvas...");

    let data_url = page
      .evaluate("document.getElementsByTagName('canvas')[0].toDataURL()")
      .await?
      .into_value::<String>()?;

    let have = image::load_from_memory(&base64::decode(&data_url[22..])?)?;

    let want = ImageReader::open(format!("images/{}.png", self.filename))?.decode()?;

    if have != want {
      let destination = format!("images/{}.browser-actual-memory.png", self.filename);
      have.save(&destination)?;
      #[cfg(target_os = "macos")]
      {
        let status = Command::new("xattr")
          .args(["-wx", "com.apple.FinderInfo"])
          .arg("0000000000000000000C00000000000000000000000000000000000000000000")
          .arg(&destination)
          .status()?;

        if !status.success() {
          panic!("xattr failed: {}", status);
        }
      }

      panic!("Images aren't the same");
    }

    Ok(())
  }
}

pub async fn browser_test(name: &str, program: &str) -> Result {
  Test::new().filename(name).program(program).run().await
}
