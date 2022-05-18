use {
  executable_path::executable_path,
  pretty_assertions::assert_eq,
  std::{fs, path::Path, process::Command, str, thread, time::Duration},
  tempfile::TempDir,
  unindent::Unindent,
};

mod image_tests;

type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

struct Test<'a> {
  env_vars: Vec<(&'a str, &'a str)>,
  expected_status: i32,
  expected_stderr: String,
  program: String,
  tempdir: TempDir,
}

impl<'a> Test<'a> {
  fn new() -> Result<Self> {
    let tempdir = TempDir::new()?;

    fs::write(tempdir.path().join("program.degen"), "x apply")?;

    Ok(Self {
      env_vars: Vec::new(),
      expected_status: 0,
      expected_stderr: String::new(),
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

  fn expected_status(self, expected_status: i32) -> Self {
    Self {
      expected_status,
      ..self
    }
  }

  fn expected_stderr(self, expected_stderr: &str) -> Self {
    Self {
      expected_stderr: expected_stderr.unindent(),
      ..self
    }
  }

  fn env_var(mut self, key: &'a str, value: &'a str) -> Self {
    self.env_vars.push((key, value));
    self
  }

  fn run(self) -> Result {
    self.run_and_return_tempdir().map(|_| ())
  }

  fn run_with_timeout(self, timeout: Duration) -> Result {
    let mut child = self.command().spawn()?;

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

  fn command(&self) -> Command {
    let mut command = Command::new(executable_path("degenerate"));

    command
      .envs(self.env_vars.iter().cloned())
      .current_dir(&self.tempdir)
      .args(self.program.split_whitespace());

    command
  }

  fn run_and_return_tempdir(self) -> Result<TempDir> {
    let output = self.command().output()?;

    let stderr = str::from_utf8(&output.stderr)?;

    assert_eq!(
      output.status.code(),
      Some(self.expected_status),
      "Program `{}` failed: {}",
      self.program,
      stderr,
    );

    if self.expected_stderr.is_empty() {
      if !stderr.is_empty() {
        panic!("Expected empty stderr:\n{}", stderr);
      }
    } else {
      assert_eq!(stderr, self.expected_stderr);
    }

    Ok(self.tempdir)
  }
}

#[test]
fn save_invalid_format() -> Result {
  Test::new()?
    .program("resize:4:4 top save:output.txt")
    .expected_status(1)
    .expected_stderr(
      "
      error: The file extension `.\"txt\"` was not recognized as an image format
      ",
    )
    .run()
}

#[test]
fn verbose_toggles_step_status() -> Result {
  Test::new()?
    .program("verbose square verbose square")
    .expected_stderr(
      "
      PC 1 LC 0 M All C Mask(Square)
      PC 2 LC 0 M Square C Verbose
      ",
    )
    .run()
}

#[test]
fn infinite_loop() -> Result {
  Test::new()?
    .program("loop")
    .run_with_timeout(Duration::from_millis(250))
}

#[test]
fn errors_printed_in_red_and_bold() -> Result<()> {
  Test::new()?
    .program("invalid")
    .env_var("CLICOLOR_FORCE", "1")
    .expected_status(1)
    .expected_stderr(
      "
      \u{1b}[1;31merror\u{1b}[0m\u{1b}[1m: Invalid command: invalid\u{1b}[0m
      ",
    )
    .run()
}
