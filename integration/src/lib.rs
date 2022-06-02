use {
  chromiumoxide::browser::BrowserConfig,
  futures::StreamExt,
  lazy_static::lazy_static,
  std::{
    fs,
    net::SocketAddr,
    path::PathBuf,
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

    page
      .evaluate(format!("window.test(`{}`)", program.unindent()))
      .await?;

    let data_url = loop {
      if let Some(data_url) = page.evaluate("window.dataURL").await?.value() {
        break data_url.as_str().ok_or("Failed to convert")?.to_owned();
      }

      tokio::time::sleep(Duration::from_millis(100)).await;
    };

    let have = image::load_from_memory(&base64::decode(
      &data_url["data:image/png;base64,".len()..],
    )?)?;

    let want_path = PathBuf::from(format!("../images/{}.png", name));

    let missing = !want_path.is_file();

    if missing || have != image::open(&want_path)? {
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

      if missing {
        panic!(
          "Image test failed:\nExpected image missing: {}\nActual:   {}",
          want_path.display(),
          destination,
        );
      } else {
        panic!(
          "Image test failed:\nExpected: {}\nActual:   {}",
          want_path.display(),
          destination,
        );
      }
    }

    Ok(())
  })
}

#[test]
fn all() -> Result {
  image_test(
    "all",
    "
      computer.all();
      computer.apply();
    ",
  )
}

#[test]
fn alpha() -> Result {
  image_test(
    "alpha",
    "
      computer.alpha(0.5);
      computer.x();
      computer.apply();
    ",
  )
}

#[test]
fn apply() -> Result {
  image_test(
    "apply",
    "
      computer.apply();
    ",
  )
}

#[test]
fn brilliance() -> Result {
  image_test(
    "brilliance",
    "
      computer.x();
      computer.rotateColor('g', 0.07);
      computer.rotate(0.07);
      for (let i = 0; i < 10; i++) {
        computer.apply();
      }
      computer.rotateColor('b', 0.09);
      computer.rotate(0.09);
      for (let i = 0; i < 10; i++) {
        computer.apply();
      }
    ",
  )
}

#[test]
fn carpet() -> Result {
  image_test(
    "carpet",
    "
      computer.circle();
      computer.scale(0.5);
      for (let i = 0; i < 8; i++) {
        computer.apply();
        computer.wrap();
      }
    ",
  )
}

#[test]
fn circle() -> Result {
  image_test(
    "circle",
    "
      computer.circle();
      computer.apply();
    ",
  )
}

#[test]
fn circle_scale() -> Result {
  image_test(
    "circle_scale",
    "
      computer.scale(0.5);
      computer.circle();
      computer.apply();
      computer.all();
      computer.scale(0.9);
      computer.wrap();
      computer.apply();
    ",
  )
}

#[test]
fn concentric_circles() -> Result {
  image_test(
    "concentric_circles",
    "
      computer.scale(0.99);
      computer.circle();
      for (let i = 0; i < 100; i++) {
        computer.apply();
      }
    ",
  )
}

#[test]
fn cross() -> Result {
  image_test(
    "cross",
    "
      computer.cross();
      computer.apply();
    ",
  )
}

#[test]
fn default_program() -> Result {
  image_test("default_program", "")
}

#[test]
fn diamonds() -> Result {
  image_test(
    "diamonds",
    "
      computer.rotate(0.3333);
      computer.rotateColor('g', 0.05);
      computer.circle();
      computer.scale(0.5);
      computer.wrap();
      for (let i = 0; i < 8; i++) {
        computer.apply();
      }
      computer.rotate(0.8333);
      computer.rotateColor('b', 0.05);
      for (let i = 0; i < 8; i++) {
        computer.apply();
      }
    ",
  )
}

#[test]
fn grain() -> Result {
  image_test(
    "grain",
    "
      computer.rotate(0.111);
      for (let i = 0; i < 16; i++) {
        computer.square();
        computer.apply();
        computer.circle();
        computer.apply();
      }
    ",
  )
}

#[test]
fn kaleidoscope() -> Result {
  image_test(
    "kaleidoscope",
    "
      computer.rotateColor('g', 0.05);
      computer.circle();
      computer.scale(0.75);
      computer.wrap();
      for (let i = 0; i < 8; i++) {
        computer.apply();
      }
      computer.rotate(0.8333);
      computer.rotateColor('b', 0.05);
      for (let i = 0; i < 8; i++) {
        computer.apply();
      }
    ",
  )
}

#[test]
fn mod_3() -> Result {
  image_test(
    "mod_3",
    "
      computer.mod(3, 0);
      computer.apply();
    ",
  )
}

#[test]
fn orbs() -> Result {
  image_test(
    "orbs",
    "
      computer.rotateColor('g', 0.05);
      computer.circle();
      computer.scale(0.75);
      computer.wrap();
      for (let i = 0; i < 8; i++) {
        computer.apply();
      }
      computer.rotateColor('b', 0.05);
      for (let i = 0; i < 8; i++) {
        computer.apply();
      }
    ",
  )
}

#[test]
fn pattern() -> Result {
  image_test(
    "pattern",
    "
      computer.alpha(0.75);
      computer.circle();
      computer.scale(0.5);
      for (let i = 0; i < 8; i++) {
        computer.apply();
        computer.wrap();
      }
    ",
  )
}

// TODO
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

// TODO
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

// TODO
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
      computer.rotate(0.05);
      computer.x();
      computer.apply();
    ",
  )
}

#[test]
fn rotate_0125_square() -> Result {
  image_test(
    "rotate_0125_square",
    "
      computer.rotate(0.125);
      computer.square();
      computer.apply();
    ",
  )
}

#[test]
fn rotate_1_square() -> Result {
  image_test(
    "rotate_1_square",
    "
      computer.rotate(1.0);
      computer.square();
      computer.apply();
    ",
  )
}

#[test]
fn rotate_color_05_red() -> Result {
  image_test(
    "rotate_color_05_red",
    "
      computer.rotateColor('red', 0.5);
      computer.all();
      computer.apply();
    ",
  )
}

#[test]
fn rotate_color_blue_05_all() -> Result {
  image_test(
    "rotate_color_blue_05_all",
    "
      computer.rotateColor('blue', 0.5);
      computer.all();
      computer.apply();
    ",
  )
}

#[test]
fn rotate_color_blue_1_all() -> Result {
  image_test(
    "rotate_color_blue_1_all",
    "
      computer.rotateColor('blue', 1.0);
      computer.all();
      computer.apply();
    ",
  )
}

#[test]
fn rotate_color_blue_all() -> Result {
  image_test(
    "rotate_color_blue_all",
    "
      computer.rotateColor('b', 0.5);
      computer.all();
      computer.apply();
    ",
  )
}

#[test]
fn rotate_color_g() -> Result {
  image_test(
    "rotate_color_g",
    "
      computer.rotateColor('g', 0.5);
      computer.all();
      computer.apply();
    ",
  )
}

#[test]
fn rotate_color_green() -> Result {
  image_test(
    "rotate_color_green",
    "
      computer.rotateColor('green', 0.5);
      computer.all();
      computer.apply();
    ",
  )
}

#[test]
fn rotate_color_green_all() -> Result {
  image_test(
    "rotate_color_green_all",
    "
      computer.rotateColor('green', 1.0);
      computer.all();
      computer.apply();
    ",
  )
}

#[test]
fn rotate_color_r() -> Result {
  image_test(
    "rotate_color_r",
    "
      computer.rotateColor('r', 0.5);
      computer.all();
      computer.apply();
    ",
  )
}

#[test]
fn rotate_color_red_all() -> Result {
  image_test(
    "rotate_color_red_all",
    "
      computer.rotateColor('red', 1.0);
      computer.all();
      computer.apply();
    ",
  )
}

#[test]
fn rotate_scale_x() -> Result {
  image_test(
    "rotate_scale_x",
    "
      computer.rotate(0.05);
      comuter.scale(2);
      computer.x();
      computer.apply();
    ",
  )
}

#[test]
fn rotate_square() -> Result {
  image_test(
    "rotate_square",
    "
      computer.rotate(0.05);
      computer.square();
      for (let i = 0; i < 2; i++) {
        computer.apply();
      }
    ",
  )
}

#[test]
fn rotate_square_for_x() -> Result {
  image_test(
    "rotate_square_for_x",
    "
      computer.rotate(0.05);
      computer.square();
      for (let i = 0; i < 2; i++) {
        computer.apply();
      }
      computer.x();
      for (let i = 0; i < 1; i++) {
        computer.apply();
      }
    ",
  )
}

#[test]
fn rows() -> Result {
  image_test(
    "rows",
    "
      computer.rows(1, 1);
      computer.apply();
    ",
  )
}

#[test]
fn rows_overflow() -> Result {
  image_test(
    "rows_overflow",
    "
      computer.rows(4294967295, 4294967295);
      computer.apply();
    ",
  )
}

#[test]
fn rug() -> Result {
  image_test(
    "rug",
    "
      computer.rotateColor('g', 0.05);
      computer.circle();
      computer.scale(0.5);
      computer.wrap();
      for (let i = 0; i < 8; i++) {
        computer.apply();
      }
      computer.rotateColor('b', 0.05);
      for (let i = 0; i < 8; i++) {
        computer.apply();
      }
    ",
  )
}

#[test]
fn scale() -> Result {
  image_test(
    "scale",
    "
      computer.scale(0.5);
      computer.circle();
      computer.apply();
    ",
  )
}

#[test]
fn scale_circle_for() -> Result {
  image_test(
    "scale_circle_for",
    "
      computer.circle();
      computer.scale(0.5);
      for (let i = 0; i < 8; i++) {
        computer.apply();
      }
    ",
  )
}

#[test]
fn scale_circle_wrap() -> Result {
  image_test(
    "scale_circle_wrap",
    "
      comuter.scale(0.5);
      computer.circle();
      computer.wrap();
      computer.apply();
    ",
  )
}

#[test]
fn scale_rotate() -> Result {
  image_test(
    "scale_rotate",
    "
      computer.scale(2);
      computer.rotate(0.05);
      computer.x();
      computer.apply();
    ",
  )
}

#[test]
fn scale_x() -> Result {
  image_test(
    "scale_x",
    "
      computer.scale(2);
      computer.x();
      computer.apply();
    ",
  )
}

// TODO
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
      computer.square();
      computer.apply();
    ",
  )
}

#[test]
fn square_top() -> Result {
  image_test(
    "square_top",
    "
      computer.square();
      computer.apply();
      computer.top();
      computer.apply();
   ",
  )
}

// TODO
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
      computer.top();
      computer.apply();
    ",
  )
}

#[test]
fn x() -> Result {
  image_test(
    "x",
    "
      computer.x();
      computer.apply();
    ",
  )
}

#[test]
fn x_loop() -> Result {
  image_test(
    "x_loop",
    "
      computer.x();
      computer.scale(0.5);
      for (let i = 0; i < 8; i++) {
        computer.apply();
        computer.wrap();
      }
    ",
  )
}

#[test]
fn x_scale() -> Result {
  image_test(
    "x_scale",
    "
      computer.x();
      computer.scale(0.5);
      for (let i = 0; i < 8; i++) {
        computer.apply();
      }
    ",
  )
}

#[test]
fn x_wrap() -> Result {
  image_test(
    "x_wrap",
    "
      computer.x();
      computer.apply();
      computer.scale(0.5);
      computer.wrap();
      computer.identity();
      computer.all();
      comuter.apply();
    ",
  )
}

#[test]
fn debug_operation() -> Result {
  image_test(
    "debug_operation",
    "
      computer.debug();
      computer.apply();
    ",
  )
}

#[test]
fn mod_zero_is_always_false() -> Result {
  image_test(
    "mod_zero_is_always_false",
    "
      computer.mod(0, 1);
      computer.apply();
    ",
  )
}

#[test]
fn square_colors() -> Result {
  image_test(
    "square_colors",
    "
      computer.rotate(0.01);
      computer.rotateColor('g', 0.1);
      computer.square();
      for (let i = 0; i < 10; i++) {
        computer.apply();
      }
    ",
  )
}

#[test]
fn nested_for_loops() -> Result {
  image_test(
    "nested_for_loops",
    "
      computer.circle();
      computer.scale(0.9);
      for (let i = 0; i < 2; i++) {
        for (let j = 0; j < 2; j++) {
          computer.apply();
        }
      }
    ",
  )
}

#[test]
fn for_zero() -> Result {
  image_test(
    "for_zero",
    "
      computer.circle();
      for (let i = 0; i < 0; i++) {
        computer.apply();
      }
    ",
  )
}

#[test]
fn gpu_extra_pixels() -> Result {
  image_test(
    "gpu_extra_pixels",
    "
      computer.rotate(0.01);
      computer.apply();
      computer.apply();
    ",
  )
}

#[test]
fn default_color() -> Result {
  image_test(
    "default_color",
    "
      computer.defaultColor([255, 0, 255]);
      computer.rotate(0.01);
      computer.apply();
    ",
  )
}
