use {
  executable_path::executable_path,
  pretty_assertions::assert_eq,
  std::{process::Command, str},
};

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn assert_output_eq(args: &[&str], expected_bitmap: &str) -> Result<()> {
  let mut command = Command::new(executable_path("degenerate"));

  command.args(args);

  let output = command.output()?;

  assert!(
    output.status.success(),
    "Command {:?} failed: {}",
    command,
    str::from_utf8(&output.stderr)?
  );

  assert_eq!(
    str::from_utf8(&output.stdout)?,
    expected_bitmap.replace(" ", "")
  );

  Ok(())
}

#[test]
fn even() -> Result<()> {
  assert_output_eq(
    &["resize:4:4", "even"],
    "1111
     0000
     1111
     0000",
  )
}

#[test]
fn top() -> Result<()> {
  assert_output_eq(
    &["resize:2:2", "top"],
    "11
     00",
  )
}

#[test]
fn resize() -> Result<()> {
  assert_output_eq(&["resize:2:1"], "00")
}

#[test]
fn invert() -> Result<()> {
  assert_output_eq(&["resize:1:1", "invert"], "1")
}

#[test]
fn square() -> Result<()> {
  assert_output_eq(
    &["resize:4:4", "square"],
    "0000
     0110
     0110
     0000",
  )
}
