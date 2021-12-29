use {
  super::*,
  executable_path::executable_path,
  pretty_assertions::assert_eq,
  std::time::Duration,
  std::{
    fs,
    io::prelude::*,
    process::{Command, Stdio},
    str, thread,
  },
  tempfile::TempDir,
  unindent::Unindent,
};

pub(crate) type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub(crate) struct Test {
  expected_status: i32,
  expected_stderr: String,
  expected_stdout: String,
  program: String,
  tempdir: TempDir,
}

impl Test {
  pub(crate) fn new() -> Result<Self> {
    Ok(Self {
      expected_status: 0,
      expected_stderr: String::new(),
      expected_stdout: String::new(),
      program: String::new(),
      tempdir: TempDir::new()?,
    })
  }

  pub(crate) fn program(self, program: impl AsRef<str>) -> Self {
    Self {
      program: program.as_ref().to_owned(),
      ..self
    }
  }

  pub(crate) fn expected_status(self, expected_status: i32) -> Self {
    Self {
      expected_status,
      ..self
    }
  }

  pub(crate) fn expected_stderr(self, expected_stderr: &str) -> Self {
    Self {
      expected_stderr: expected_stderr.unindent(),
      ..self
    }
  }

  pub(crate) fn expected_stdout(self, expected_stdout: &str) -> Self {
    Self {
      expected_stdout: expected_stdout.unindent(),
      ..self
    }
  }

  pub(crate) fn run(self) -> Result<()> {
    self.run_and_return_tempdir().map(|_| ())
  }

  pub(crate) fn run_with_timeout(self, timeout: Duration) -> Result<()> {
    let mut child = Command::new(executable_path("degenerate"))
      .current_dir(&self.tempdir)
      .args(self.program.split_whitespace())
      .spawn()?;

    thread::sleep(timeout);

    if let Some(status) = child.try_wait()? {
      panic!(
        "program `{}` exited before timeout elapsed: {}",
        self.program, status
      );
    }

    child.kill()?;

    Ok(())
  }

  pub(crate) fn run_and_return_tempdir(self) -> Result<TempDir> {
    let output = Command::new(executable_path("degenerate"))
      .current_dir(&self.tempdir)
      .args(self.program.split_whitespace())
      .output()?;

    let stderr = str::from_utf8(&output.stderr)?;

    assert_eq!(
      output.status.code(),
      Some(self.expected_status),
      "Program `{}` failed: {}",
      self.program,
      stderr,
    );

    assert_eq!(stderr, self.expected_stderr);

    assert_eq!(str::from_utf8(&output.stdout)?, self.expected_stdout);

    Ok(self.tempdir)
  }
}

pub(crate) fn image_test(program: &str) -> Result<()> {
  let tempdir = Test::new()?.program(program).run_and_return_tempdir()?;

  let actual_path = tempdir.path().join("output.png");

  let actual_image = image::open(&actual_path)?;

  let expected_path = format!("images/{}.png", program);
  let expected_image = image::open(&expected_path)?;

  if actual_image != expected_image {
    let destination = format!("images/{}.actual-output.png", program);
    fs::rename(&actual_path, &destination)?;
    panic!(
      "Image test failed:\nExpected: {}\nActual:   {}",
      expected_path, destination,
    );
  }

  Ok(())
}
