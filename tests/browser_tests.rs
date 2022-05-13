use {
  async_std::task, base64::decode, chromiumoxide::browser::BrowserConfig,
  executable_path::executable_path, futures::StreamExt, image::io::Reader as ImageReader,
  std::process::Command, tempfile::TempDir,
};

const URL: &'static str = "https://degenerate.computer";

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

struct Browser {
  inner: chromiumoxide::Browser,
  _handle: task::JoinHandle<()>,
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

    Ok(Browser {
      inner,
      _handle: handle,
    })
  }
}

struct Test<'a> {
  env_vars: Vec<(&'a str, &'a str)>,
  program: String,
  tempdir: TempDir,
}

impl<'a> Test<'a> {
  fn new() -> Result<Self> {
    let tempdir = TempDir::new()?;

    Ok(Self {
      env_vars: Vec::new(),
      program: String::new(),
      tempdir,
    })
  }

  fn program(self, program: impl AsRef<str>) -> Self {
    Self {
      program: program.as_ref().to_owned(),
      ..self
    }
  }

  fn command(&self) -> Result {
    let mut command = Command::new(executable_path("degenerate"));

    command
      .envs(self.env_vars.iter().cloned())
      .current_dir(&self.tempdir)
      .args(self.program.split_whitespace())
      .spawn()?;

    Ok(())
  }

  async fn run(&self) -> Result {
    let page = Browser::new().await?.inner.new_page(URL).await?;

    page
      .find_elements("textarea")
      .await?
      .first()
      .ok_or("Could not find `textarea` element")?
      .click()
      .await?
      .type_str(self.program.clone())
      .await?;

    let data_url = page
      .evaluate("document.getElementsByTagName('canvas')[0].toDataURL()")
      .await?
      .into_value::<String>()?;

    let have = image::load_from_memory(&decode(&data_url[22..])?)?;

    self.command()?;

    let want = ImageReader::open(self.tempdir.path().join("memory.png"))
      .unwrap()
      .decode()?;

    assert_eq!(have, want);

    Ok(())
  }
}

#[test]
fn circle() {
  task::block_on(async {
    Test::new()
      .unwrap()
      .program("circle apply save")
      .run()
      .await
      .unwrap();
  })
}
