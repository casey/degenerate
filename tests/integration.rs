use {
  executable_path::executable_path,
  pretty_assertions::assert_eq,
  std::{
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

  fn program(self, program: &str) -> Self {
    Self {
      program: program.to_owned(),
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
fn circle() -> Result<()> {
  Test::new()?
    .program("resize:10:10 circle print")
    .expected_stdout(
      "
      000FFFF000
      0FFFFFFFF0
      0FFFFFFFF0
      FFFFFFFFFF
      FFFFFFFFFF
      FFFFFFFFFF
      FFFFFFFFFF
      0FFFFFFFF0
      0FFFFFFFF0
      000FFFF000
      ",
    )
    .run()
}

#[test]
fn even() -> Result<()> {
  Test::new()?
    .program("resize:4:4 even print")
    .expected_stdout(
      "
      FFFF
      0000
      FFFF
      0000
      ",
    )
    .run()
}

#[test]
fn top() -> Result<()> {
  Test::new()?
    .program("resize:2:2 top print")
    .expected_stdout(
      "
      FF
      00
      ",
    )
    .run()
}

#[test]
fn repl_valid_filter() -> Result<()> {
  let mut command = Command::new(executable_path("degenerate"))
    .args(["resize:4:4", "repl"])
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()?;

  write!(command.stdin.as_mut().unwrap(), "even")?;

  assert_eq!(
    str::from_utf8(&command.wait_with_output()?.stdout)?,
    "FFFF\n0000\nFFFF\n0000\n"
  );

  Ok(())
}

#[test]
fn repl_invalid_filter() -> Result<()> {
  let mut command = Command::new(executable_path("degenerate"))
    .args(["resize:4:4", "repl", "print"])
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()?;

  let stdin = command.stdin.as_mut().unwrap();
  write!(stdin, "invalid")?;

  assert_eq!(
    str::from_utf8(&command.wait_with_output()?.stdout)?,
    "0000\n0000\n0000\n0000\n"
  );

  Ok(())
}

#[test]
fn resize() -> Result<()> {
  Test::new()?
    .program("resize:2:1 print")
    .expected_stdout(
      "
      00
      ",
    )
    .run()
}

#[test]
fn invert() -> Result<()> {
  Test::new()?
    .program("resize:1:1 all print")
    .expected_stdout(
      "
      F
      ",
    )
    .run()
}

#[test]
fn save() -> Result<()> {
  let tempdir = Test::new()?
    .program("resize:1:2 top save:output.png print")
    .expected_stdout(
      "
      F
      0
      ",
    )
    .run_and_return_tempdir()?;

  let image = image::open(tempdir.path().join("output.png"))?
    .as_rgb8()
    .unwrap()
    .to_owned();
  assert_eq!(image.width(), 1);
  assert_eq!(image.height(), 2);
  assert_eq!(image.to_vec(), &[255, 255, 255, 0, 0, 0]);

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
fn rows() -> Result<()> {
  assert_output_eq(
    &["resize:4:4", "rows:2:1"],
    "1111
     0000
     1111
     0000",
  )
}

#[test]
fn rows_invalid_number_of_rows() -> Result<()> {
  assert_output_eq(
    &["resize:4:4", "rows:3:1"],
    "1111
     0000
     1111
     0000",
  )
}

#[test]
fn square() -> Result<()> {
  Test::new()?
    .program("resize:4:4 square print")
    .expected_stdout(
      "
      0000
      0FF0
      0FF0
      0000
      ",
    )
    .run()
}

#[test]
fn load() -> Result<()> {
  Test::new()?
    .program("resize:1:2 save:output.png top load:output.png print")
    .expected_stdout(
      "
      0
      0
      ",
    )
    .run()
}

#[test]
fn modulus() -> Result<()> {
  Test::new()?
    .program("resize:4:2 mod:2:0 print")
    .expected_stdout(
      "
      FFFF
      0000
      ",
    )
    .run()
}

#[test]
fn default_bitmap_size() -> Result<()> {
  Test::new()?.program("print").run()
}

#[test]
fn random() -> Result<()> {
  Test::new()?
    .program("resize:4:2 random all print")
    .expected_stdout(
      "
      8569
      3275
      ",
    )
    .run()
}

#[test]
fn reset_filter() -> Result<()> {
  Test::new()?
    .program("resize:4:2 random all invert all print")
    .expected_stdout(
      "
      7A96
      CD8A
      ",
    )
    .run()
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
    .program("resize:4:4 for:2 square print loop for:1 even print loop")
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
