use {
  chromiumoxide::browser::BrowserConfig, futures::StreamExt, image::io::Reader as ImageReader,
  tokio::task,
};

const URL: &'static str = "https://degenerate.computer";

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
    let page = Browser::new().await?.inner.new_page(URL).await?;

    page
      .evaluate(format!(
        "document.getElementsByTagName('textarea')[0].innerHTML = '{}'",
        self.program
      ))
      .await?
      .into_value::<String>()?;

    let data_url = page
      .evaluate("document.getElementsByTagName('canvas')[0].toDataURL()")
      .await?
      .into_value::<String>()?;

    eprintln!("testing");

    assert_eq!(
      image::load_from_memory(&base64::decode(&data_url[22..])?)?,
      ImageReader::open(format!("images/{}.png", self.filename))?.decode()?
    );

    Ok(())
  }
}

#[tokio::test]
async fn circle() -> Result {
  Test::new()
    .filename("circle")
    .program("circle apply save")
    .run()
    .await
}
