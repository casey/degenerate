use {
  executable_path::executable_path,
  pretty_assertions::assert_eq,
  std::{
    fs,
    io::prelude::*,
    process::{Command, Stdio},
    str, thread,
    time::Duration,
  },
  tempfile::TempDir,
  unindent::Unindent,
};

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

struct Test {
  expected_status: i32,
  expected_stderr: String,
  expected_stdout: String,
  program: String,
  tempdir: TempDir,
}

impl Test {
  fn new() -> Result<Self> {
    Ok(Self {
      expected_status: 0,
      expected_stderr: String::new(),
      expected_stdout: String::new(),
      program: String::new(),
      tempdir: TempDir::new()?,
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

  fn expected_stdout(self, expected_stdout: &str) -> Self {
    Self {
      expected_stdout: expected_stdout.unindent(),
      ..self
    }
  }

  fn run(self) -> Result<()> {
    self.run_and_return_tempdir().map(|_| ())
  }

  fn run_with_timeout(self, timeout: Duration) -> Result<()> {
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

  fn run_and_return_tempdir(self) -> Result<TempDir> {
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

#[test]
fn repl_returns_success_after_reaching_eol() -> Result<()> {
  Test::new()?.program("repl").run()
}

#[test]
fn repl_valid_filter() -> Result<()> {
  let mut command = Command::new(executable_path("degenerate"))
    .args(["resize:4:4", "repl"])
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()?;

  write!(command.stdin.as_mut().unwrap(), "rows:1:1")?;

  assert_eq!(
    str::from_utf8(&command.wait_with_output()?.stdout)?,
    "FFFF\n0000\nFFFF\n0000\n"
  );

  Ok(())
}

#[test]
fn repl_invalid_filter() -> Result<()> {
  let mut command = Command::new(executable_path("degenerate"))
    .args(["resize:4:4", "repl"])
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()?;

  write!(command.stdin.as_mut().unwrap(), "invalid")?;

  assert_eq!(
    str::from_utf8(&command.wait_with_output()?.stderr)?,
    "Could not parse command from `invalid`: Invalid command: invalid\n"
  );

  Ok(())
}

#[test]
fn save_invalid_format() -> Result<()> {
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
fn default_bitmap_size() -> Result<()> {
  Test::new()?.program("print").run()
}

#[test]
fn verbose_toggles_step_status() -> Result<()> {
  Test::new()?
    .program("verbose square verbose square")
    .expected_stderr(
      "
      PC 1 LC 0 Filter(Square)
      PC 2 LC 0 Verbose
      ",
    )
    .run()
}

#[test]
fn looping() -> Result<()> {
  Test::new()?
    .program("resize:4:4 for:2 square print loop")
    .expected_stdout(
      "
      0000
      0FF0
      0FF0
      0000
      0000
      0000
      0000
      0000
    ",
    )
    .run()
}

#[test]
fn multiple_fors_reset_loop_counter() -> Result<()> {
  Test::new()?
    .program("resize:4:4 for:2 square print loop for:1 rows:1:1 print loop")
    .expected_stdout(
      "
      0000
      0FF0
      0FF0
      0000
      0000
      0000
      0000
      0000
      FFFF
      0000
      FFFF
      0000
      ",
    )
    .run()
}

#[test]
fn infinite_loop() -> Result<()> {
  Test::new()?
    .program("loop")
    .run_with_timeout(Duration::from_millis(250))
}

#[test]
fn image_tests() -> Result<()> {
  for result in fs::read_dir("images")? {
    let entry = result?;
    eprintln!("Running image test on {}…", entry.path().display());

    let filename = entry
      .file_name()
      .into_string()
      .map_err(|filename| format!("Could not convert filename to unicode: {:?}", filename))?;

    if !filename.ends_with(".png") || filename.ends_with(".actual-output.png") {
      continue;
    }

    let expected_path = entry.path();

    let expected_image = image::open(&expected_path)?;

    let program = expected_path
      .file_stem()
      .ok_or_else(|| format!("Could not extract file stem: {}", expected_path.display()))?
      .to_str()
      .ok_or_else(|| format!("Path was not valid UTF-8: {}", expected_path.display()))?;

    let tempdir = Test::new()?.program(program).run_and_return_tempdir()?;

    let actual_path = tempdir.path().join("output.png");

    let actual_image = image::open(&actual_path)?;

    if actual_image != expected_image {
      let destination = format!("images/{}.actual-output.png", program);
      fs::rename(&actual_path, &destination)?;
      panic!(
        "Image test failed:\nExpected: {}\nActual:   {}",
        expected_path.display(),
        destination,
      );
    }
  }
  Ok(())
}
