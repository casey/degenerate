use {
  axum::{http::StatusCode, routing::get_service, Router},
  chromiumoxide::browser::{Browser, BrowserConfig},
  futures::StreamExt,
  lazy_static::lazy_static,
  std::{fs, net::SocketAddr, process::Command, str, sync::Once, time::Duration},
  tokio::{runtime::Runtime, task},
  tower_http::{services::ServeDir, trace::TraceLayer},
  tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt},
};

macro_rules! image_test {
  (
    name: $name:ident,
    program: $program:literal,
  ) => {
    #[test]
    fn $name() -> Result {
      image_test(stringify!($name), $program)
    }
  };
}

type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

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

    let addr = SocketAddr::from(([127, 0, 0, 1], 0));
    let listener = std::net::TcpListener::bind(addr).unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);

    RUNTIME.spawn(async move {
      let addr = SocketAddr::from(([127, 0, 0, 1], port));
      tracing::trace!("Listening on {}", addr);

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

      let server = axum::Server::bind(&addr).serve(app.into_make_service());

      task::spawn(async move { server.await.unwrap() });
    });

    port
  };
}

fn clean() {
  static ONCE: Once = Once::new();

  ONCE.call_once(|| {
    for result in fs::read_dir("../images").unwrap() {
      let entry = result.unwrap();
      let path = entry.path();
      let file_name = path.file_name().unwrap().to_str().unwrap().to_string();

      if file_name.ends_with(".native-actual-memory.png")
        || file_name.ends_with(".browser-actual-memory.png")
      {
        fs::remove_file(path).unwrap();
      }
    }
  });
}

pub(crate) fn image_test(name: &str, program: &str) -> Result {
  let browser: &'static Browser = &*BROWSER;
  RUNTIME.block_on(async {
    clean();

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

    let want_path = format!("../images/{}.png", name);

    let want = image::open(&want_path)?;

    if have != want {
      let destination = format!("../images/{}.browser-actual-memory.png", name);

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

      panic!(
        "Image test failed:\nExpected: {}\nActual:   {}",
        want_path, destination,
      );
    }

    Ok(())
  })
}

image_test! {
  name: all,
  program: "all apply",
}

image_test! {
  name: alpha,
  program: "alpha:0.5 x apply",
}

image_test! {
  name: apply,
  program: "apply",
}

image_test! {
  name: brilliance,
  program: "x rotate-color:g:0.07 rotate:0.07 for:10 apply loop rotate-color:b:0.09 rotate:0.09 for:10 apply loop",
}

image_test! {
  name: carpet,
  program: "circle scale:0.5 for:8 apply wrap loop",
}

image_test! {
  name: circle,
  program: "circle apply",
}

image_test! {
  name: circle_scale,
  program: "scale:0.5 circle apply all scale:0.9 wrap apply",
}

image_test! {
  name: concentric_circles,
  program: "scale:0.99 circle for:100 apply loop",
}

image_test! {
  name: cross,
  program: "cross apply",
}

image_test! {
  name: default,
  program: "comment:foo",
}

image_test! {
  name: diamonds,
  program: "rotate:0.3333 rotate-color:g:0.05 circle scale:0.5 wrap for:8 apply loop rotate:0.8333 rotate-color:b:0.05 for:8 apply loop",
}

image_test! {
  name: grain,
  program: "rotate:0.111 for:16 square apply circle apply loop",
}

image_test! {
  name: kaleidoscope,
  program: "rotate-color:g:0.05 circle scale:0.75 wrap for:8 apply loop rotate:0.8333 rotate-color:b:0.05 for:8 apply loop",
}

image_test! {
  name: mod_3,
  program: "mod:3:0 apply",
}

image_test! {
  name: orbs,
  program: "rotate-color:g:0.05 circle scale:0.75 wrap for:8 apply loop rotate-color:b:0.05 for:8 apply loop",
}

image_test! {
  name: pattern,
  program: "alpha:0.75 circle scale:0.5 for:8 apply wrap loop",
}

image_test! {
  name: choose_default_seed,
  program: "choose:all:circle:cross:square:top:x apply",
}

image_test! {
  name: choose_zero_seed,
  program: "choose:all:circle:cross:square:top:x apply",
}

image_test! {
  name: choose_nonzero_seed,
  program: "seed:2 choose:all:circle:cross:square:top:x apply",
}

image_test! {
  name: rotate,
  program: "rotate:0.05 x apply",
}

image_test! {
  name: rotate_0125_square,
  program: "rotate:0.125 square apply",
}

image_test! {
  name: rotate_1_square,
  program: "rotate:1.0 square apply",
}

image_test! {
  name: rotate_color_05_red,
  program: "rotate-color:red:0.5 all apply",
}

image_test! {
  name: rotate_color_blue_05_all,
  program: "rotate-color:blue:0.5 all apply",
}

image_test! {
  name: rotate_color_blue_1_all,
  program: "rotate-color:blue:1.0 all apply",
}

image_test! {
  name: rotate_color_blue_all,
  program: "rotate-color:b:0.5 all apply",
}

image_test! {
  name: rotate_color_g,
  program: "rotate-color:g:0.5 all apply",
}

image_test! {
  name: rotate_color_green,
  program: "rotate-color:green:0.5 all apply",
}

image_test! {
  name: rotate_color_green_all,
  program: "rotate-color:green:1.0 all",
}

image_test! {
  name: rotate_color_r,
  program: "rotate-color:r:0.5 all apply",
}

image_test! {
  name: rotate_color_red_all,
  program: "rotate-color:red:1.0 all",
}

image_test! {
  name: rotate_scale_x,
  program: "rotate:0.05 scale:2 x apply",
}

image_test! {
  name: rotate_square,
  program: "rotate:0.05 square for:2 apply loop",
}

image_test! {
  name: rotate_square_for_x,
  program: "rotate:0.05 square for:2 apply loop x for:1 apply loop",
}

image_test! {
  name: rows,
  program: "rows:1:1 apply",
}

image_test! {
  name: rows_overflow,
  program: "rows:18446744073709551615:18446744073709551615 apply",
}

image_test! {
  name: rug,
  program: "rotate-color:g:0.05 circle scale:0.5 wrap for:8 apply loop rotate-color:b:0.05 for:8 apply loop",
}

image_test! {
  name: scale,
  program: "scale:0.5 circle apply",
}

image_test! {
  name: scale_circle_for,
  program: "circle scale:0.5 for:8 apply loop",
}

image_test! {
  name: scale_circle_wrap,
  program: "scale:0.5 circle wrap apply",
}

image_test! {
  name: scale_rotate,
  program: "scale:2 rotate:0.05 x apply",
}

image_test! {
  name: scale_x,
  program: "scale:2 x apply",
}

image_test! {
  name: smear,
  program: "seed:9 rotate-color:g:0.01 rotate:0.01 for:100 choose:all:circle:cross:square:top:x apply loop rotate-color:b:0.01 rotate:0.01 for:100 choose:all:circle:cross:square:top:x apply loop",
}

image_test! {
  name: square,
  program: "square apply",
}

image_test! {
  name: square_top,
  program: "square apply top apply",
}

image_test! {
  name: starburst,
  program: "seed:8 rotate-color:g:0.1 rotate:0.1 for:10 choose:all:circle:cross:square:top:x apply loop rotate-color:b:0.1 rotate:0.1 for:10 choose:all:circle:cross:square:top:x apply loop",
}

image_test! {
  name: top,
  program: "top apply",
}

image_test! {
  name: x,
  program: "x apply",
}

image_test! {
  name: x_loop,
  program: "x scale:0.5 for:8 apply wrap loop",
}

image_test! {
  name: x_scale,
  program: "x scale:0.5 for:8 apply loop",
}

image_test! {
  name: x_wrap,
  program: "x apply scale:0.5 wrap identity all apply",
}

image_test! {
  name: debug_operation,
  program: "debug apply",
}

image_test! {
  name: mod_zero_is_always_false,
  program: "mod:0:1 apply",
}

image_test! {
  name: square_colors,
  program: "rotate:0.01 rotate-color:g:0.1 square for:10 apply loop",
}
