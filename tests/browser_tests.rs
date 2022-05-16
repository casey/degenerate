use {
  chromiumoxide::browser::BrowserConfig,
  chromiumoxide::handler::viewport::Viewport,
  futures::StreamExt,
  image::io::Reader as ImageReader,
  std::{thread::sleep, time::Duration},
  tokio::task,
};

const URL: &'static str = "http://localhost:8000";

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

struct Browser {
  inner: chromiumoxide::Browser,
  handle: task::JoinHandle<()>,
}

impl Browser {
  async fn new() -> Result<Self> {
    let (inner, mut handler) = chromiumoxide::Browser::launch(
      BrowserConfig::builder()
        .arg("--allow-insecure-localhost")
        .viewport(Viewport {
          width: 256,
          height: 256,
          ..Viewport::default()
        })
        .build()?,
    )
    .await?;

    let handle = task::spawn(async move {
      loop {
        let _ = handler.next().await.unwrap();
      }
    });

    Ok(Browser { inner, handle })
  }
}

impl Drop for Browser {
  fn drop(&mut self) {
    self.handle.abort();
  }
}

struct Test {
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
    eprintln!("Launching browser...");

    let browser = Browser::new().await?;

    eprintln!("Creating page...");

    let page = browser.inner.new_page(URL).await?;
    page.wait_for_navigation().await?;

    eprintln!("Setting program on textarea...");

    page
      .find_elements("textarea")
      .await?
      .first()
      .ok_or("Could not find textarea")?
      .type_str(self.program.clone())
      .await?;

    sleep(Duration::from_secs(3));

    eprintln!("Grabbing data url from canvas...");

    let data_url = page
      .evaluate("document.getElementsByTagName('canvas')[0].toDataURL()")
      .await?
      .into_value::<String>()?;

    let have = image::load_from_memory(&base64::decode(&data_url[22..])?)?;
    have.save("have.png")?;

    let want = ImageReader::open(format!("images/{}.png", self.filename))?
      .decode()?;
    want.save("want.png")?;

    assert_eq!(have, want);

    Ok(())
  }
}

#[tokio::test]
async fn circle() -> Result {
  Test::new()
    .filename("circle")
    .program("circle apply")
    .run()
    .await
}
