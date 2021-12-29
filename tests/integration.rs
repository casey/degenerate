use {
  executable_path::executable_path,
  pretty_assertions::assert_eq,
  shared::{Result, Test},
  std::{
    fs,
    io::prelude::*,
    process::{Command, Stdio},
    str, thread,
    time::Duration,
  },
  unindent::Unindent,
};

mod shared;

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
fn default_size_is_empty() -> Result<()> {
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
