use {
  chromiumoxide::browser::BrowserConfig,
  futures::StreamExt,
  lazy_static::lazy_static,
  std::{
    fs,
    net::SocketAddr,
    str,
    sync::{Arc, Once, Weak},
    time::Duration,
  },
  tokio::{
    runtime::Runtime,
    sync::Mutex,
    task::{self, JoinHandle},
  },
  tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt},
  unindent::Unindent,
};

type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

struct Browser {
  inner: chromiumoxide::Browser,
  handle: JoinHandle<()>,
}

impl Drop for Browser {
  fn drop(&mut self) {
    self.handle.abort();
  }
}

async fn browser() -> Arc<Browser> {
  let mut guard = BROWSER.lock().await;

  if let Some(browser) = guard.upgrade() {
    return browser;
  }

  let (inner, mut handler) = chromiumoxide::Browser::launch(
    BrowserConfig::builder()
      .arg("--allow-insecure-localhost")
      .build()
      .unwrap(),
  )
  .await
  .unwrap();

  let handle = tokio::task::spawn(async move {
    loop {
      let _ = handler.next().await.unwrap();
    }
  });

  let browser = Arc::new(Browser { inner, handle });

  *guard = Arc::downgrade(&browser);

  browser
}

lazy_static! {
  static ref BROWSER: Mutex<Weak<Browser>> = Mutex::new(Weak::new());
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
      task::spawn(async move { serve::run(port).await.unwrap() });
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

      if file_name.ends_with(".actual-memory.png") {
        fs::remove_file(path).unwrap();
      }
    }
  });
}

pub(crate) fn image_test(name: &str, program: &str) -> Result {
  RUNTIME.block_on(async {
    clean();

    eprintln!("Creating page...");

    let browser = browser().await;

    let page = browser
      .inner
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
      .evaluate(format!("window.test(`{}`)", program.unindent()))
      .await?
      .into_value::<String>()?;

    let have = image::load_from_memory(&base64::decode(
      &data_url["data:image/png;base64,".len()..],
    )?)?;

    let want_path = format!("../images/{}.png", name);

    let want = image::open(&want_path)?;

    if have != want {
      let destination = format!("../images/{}.actual-memory.png", name);

      have.save(&destination)?;

      #[cfg(target_os = "macos")]
      {
        let status = std::process::Command::new("xattr")
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

#[test]
fn all() -> Result {
  image_test(
    "all",
    "
      all
      apply
    ",
  )
}

#[test]
fn alpha() -> Result {
  image_test(
    "alpha",
    "
      alpha 0.5
      x
      apply
    ",
  )
}

#[test]
fn apply() -> Result {
  image_test(
    "apply",
    "
      apply
    ",
  )
}

#[test]
fn brilliance() -> Result {
  image_test(
    "brilliance",
    "
      x
      rotate-color g 0.07
      rotate 0.07
      for 10
        apply
      loop
      rotate-color b 0.09
      rotate 0.09
      for 10
        apply
      loop
    ",
  )
}

#[test]
fn carpet() -> Result {
  image_test(
    "carpet",
    "
      circle
      scale 0.5
      for 8
        apply
        wrap
      loop
    ",
  )
}

#[test]
fn circle() -> Result {
  image_test(
    "circle",
    "
      circle
      apply
    ",
  )
}

#[test]
fn circle_scale() -> Result {
  image_test(
    "circle_scale",
    "
      scale 0.5
      circle
      apply
      all
      scale 0.9
      wrap
      apply
    ",
  )
}

#[test]
fn concentric_circles() -> Result {
  image_test(
    "concentric_circles",
    "
      scale 0.99
      circle
      for 100
        apply
      loop
    ",
  )
}

#[test]
fn cross() -> Result {
  image_test(
    "cross",
    "
      cross
      apply
    ",
  )
}

#[test]
fn default() -> Result {
  image_test("default", "")
}

#[test]
fn diamonds() -> Result {
  image_test(
    "diamonds",
    "
      rotate 0.3333
      rotate-color g 0.05
      circle
      scale 0.5
      wrap
      for 8
        apply
      loop
      rotate 0.8333
      rotate-color b 0.05
      for 8
        apply
      loop
    ",
  )
}

#[test]
fn grain() -> Result {
  image_test(
    "grain",
    "
      rotate 0.111
      for 16
        square
        apply
        circle
        apply
      loop
    ",
  )
}

#[test]
fn kaleidoscope() -> Result {
  image_test(
    "kaleidoscope",
    "
      rotate-color g 0.05
      circle
      scale 0.75
      wrap
      for 8
        apply
      loop
      rotate 0.8333
      rotate-color b 0.05
      for 8
        apply
      loop
    ",
  )
}

#[test]
fn mod_3() -> Result {
  image_test(
    "mod_3",
    "
      mod 3 0
      apply
    ",
  )
}

#[test]
fn orbs() -> Result {
  image_test(
    "orbs",
    "
      rotate-color g 0.05
      circle
      scale 0.75
      wrap
      for 8
        apply
      loop
      rotate-color b 0.05
      for 8
        apply
      loop
    ",
  )
}

#[test]
fn pattern() -> Result {
  image_test(
    "pattern",
    "
      alpha 0.75
      circle
      scale 0.5
      for 8
        apply
        wrap
      loop
    ",
  )
}

#[test]
fn choose_default_seed() -> Result {
  image_test(
    "choose_default_seed",
    "
      choose all circle cross square top x
      apply
    ",
  )
}

#[test]
fn choose_zero_seed() -> Result {
  image_test(
    "choose_zero_seed",
    "
      choose all circle cross square top x
      apply
    ",
  )
}

#[test]
fn choose_nonzero_seed() -> Result {
  image_test(
    "choose_nonzero_seed",
    "
      seed 2
      choose all circle cross square top x
      apply
    ",
  )
}

#[test]
fn rotate() -> Result {
  image_test(
    "rotate",
    "
      rotate 0.05
      x
      apply
    ",
  )
}

#[test]
fn rotate_0125_square() -> Result {
  image_test(
    "rotate_0125_square",
    "
      rotate 0.125
      square
      apply
    ",
  )
}

#[test]
fn rotate_1_square() -> Result {
  image_test(
    "rotate_1_square",
    "
      rotate 1.0
      square
      apply
    ",
  )
}

#[test]
fn rotate_color_05_red() -> Result {
  image_test(
    "rotate_color_05_red",
    "
      rotate-color red 0.5
      all
      apply
    ",
  )
}

#[test]
fn rotate_color_blue_05_all() -> Result {
  image_test(
    "rotate_color_blue_05_all",
    "
      rotate-color blue 0.5
      all
      apply
    ",
  )
}

#[test]
fn rotate_color_blue_1_all() -> Result {
  image_test(
    "rotate_color_blue_1_all",
    "
      rotate-color blue 1.0
      all
      apply
    ",
  )
}

#[test]
fn rotate_color_blue_all() -> Result {
  image_test(
    "rotate_color_blue_all",
    "
      rotate-color b 0.5
      all
      apply
    ",
  )
}

#[test]
fn rotate_color_g() -> Result {
  image_test(
    "rotate_color_g",
    "
      rotate-color g 0.5
      all
      apply
    ",
  )
}

#[test]
fn rotate_color_green() -> Result {
  image_test(
    "rotate_color_green",
    "
      rotate-color green 0.5
      all
      apply
    ",
  )
}

#[test]
fn rotate_color_green_all() -> Result {
  image_test(
    "rotate_color_green_all",
    "
      rotate-color green 1.0
      all
    ",
  )
}

#[test]
fn rotate_color_r() -> Result {
  image_test(
    "rotate_color_r",
    "
      rotate-color r 0.5
      all
      apply
    ",
  )
}

#[test]
fn rotate_color_red_all() -> Result {
  image_test(
    "rotate_color_red_all",
    "
      rotate-color red 1.0
      all
    ",
  )
}

#[test]
fn rotate_scale_x() -> Result {
  image_test(
    "rotate_scale_x",
    "
      rotate 0.05
      scale 2
      x
      apply
    ",
  )
}

#[test]
fn rotate_square() -> Result {
  image_test(
    "rotate_square",
    "
      rotate 0.05
      square
      for 2
        apply
      loop
    ",
  )
}

#[test]
fn rotate_square_for_x() -> Result {
  image_test(
    "rotate_square_for_x",
    "
      rotate 0.05
      square
      for 2
        apply
      loop
      x
      for 1
        apply
      loop
    ",
  )
}

#[test]
fn rows() -> Result {
  image_test(
    "rows",
    "
      rows 1 1
      apply
    ",
  )
}

#[test]
fn rows_overflow() -> Result {
  image_test(
    "rows_overflow",
    "
      rows 4294967295 4294967295
      apply
    ",
  )
}

#[test]
fn rug() -> Result {
  image_test(
    "rug",
    "
      rotate-color g 0.05
      circle
      scale 0.5
      wrap
      for 8
        apply
      loop
      rotate-color b 0.05
      for 8
        apply
      loop
    ",
  )
}

#[test]
fn scale() -> Result {
  image_test(
    "scale",
    "
      scale 0.5
      circle
      apply
    ",
  )
}

#[test]
fn scale_circle_for() -> Result {
  image_test(
    "scale_circle_for",
    "
      circle
      scale 0.5
      for 8
        apply
      loop
    ",
  )
}

#[test]
fn scale_circle_wrap() -> Result {
  image_test(
    "scale_circle_wrap",
    "
      scale 0.5
      circle
      wrap
      apply
    ",
  )
}

#[test]
fn scale_rotate() -> Result {
  image_test(
    "scale_rotate",
    "
      scale 2
      rotate 0.05
      x
      apply
    ",
  )
}

#[test]
fn scale_x() -> Result {
  image_test(
    "scale_x",
    "
      scale 2
      x
      apply
    ",
  )
}

#[test]
fn smear() -> Result {
  image_test(
    "smear",
    "
      seed 9
      rotate-color g 0.01
      rotate 0.01
      for 100
        choose all circle cross square top x
        apply
      loop
      rotate-color b 0.01
      rotate 0.01
      for 100
        choose all circle cross square top x
        apply
      loop
    ",
  )
}

#[test]
fn square() -> Result {
  image_test(
    "square",
    "
      square
      apply
    ",
  )
}

#[test]
fn square_top() -> Result {
  image_test(
    "square_top",
    "
    square
    apply
    top
    apply
  ",
  )
}

#[test]
fn starburst() -> Result {
  image_test(
    "starburst",
    "
      seed 8
      rotate-color g 0.1
      rotate 0.1
      for 10
        choose all circle cross square top x
        apply
      loop
      rotate-color b 0.1
      rotate 0.1
      for 10
        choose all circle cross square top x
        apply
      loop
    ",
  )
}

#[test]
fn top() -> Result {
  image_test(
    "top",
    "
      top
      apply
    ",
  )
}

#[test]
fn x() -> Result {
  image_test(
    "x",
    "
      x
      apply
    ",
  )
}

#[test]
fn x_loop() -> Result {
  image_test(
    "x_loop",
    "
      x
      scale 0.5
      for 8
        apply
        wrap
      loop
    ",
  )
}

#[test]
fn x_scale() -> Result {
  image_test(
    "x_scale",
    "
      x
      scale 0.5
      for 8
        apply
      loop
    ",
  )
}

#[test]
fn x_wrap() -> Result {
  image_test(
    "x_wrap",
    "
      x
      apply
      scale 0.5
      wrap
      identity
      all
      apply
    ",
  )
}

#[test]
fn debug_operation() -> Result {
  image_test(
    "debug_operation",
    "
      debug
      apply
    ",
  )
}

#[test]
fn mod_zero_is_always_false() -> Result {
  image_test(
    "mod_zero_is_always_false",
    "
      mod 0 1
      apply
    ",
  )
}

#[test]
fn square_colors() -> Result {
  image_test(
    "square_colors",
    "
      rotate 0.01
      rotate-color g 0.1
      square
      for 10
        apply
      loop
    ",
  )
}

#[test]
fn nested_for_loops() -> Result {
  image_test(
    "nested_for_loops",
    "
      circle
      scale 0.9

      for 2
        for 2
          apply
        loop
      loop
    ",
  )
}

#[test]
fn for_zero() -> Result {
  image_test(
    "for_zero",
    "
      circle

      for 0
        apply
      loop
    ",
  )
}

#[test]
fn gpu_extra_pixels() -> Result {
  image_test(
    "gpu_extra_pixels",
    "
      rotate 0.01
      apply
      apply
    ",
  )
}
