use {
  executable_path::executable_path,
  std::{process::Command, str},
};

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn assert_output_eq(args: &[&str], expected_output: &[&str]) -> Result<()> {
  let mut command = Command::new(executable_path("degenerate"));

  command.args(args);

  let output = command.output()?;

  assert!(
    output.status.success(),
    "Command {:?} failed: {}",
    command,
    str::from_utf8(&output.stderr)?
  );

  assert_eq!(str::from_utf8(&output.stdout)?, expected_output.join("\n"));

  Ok(())
}

#[test]
fn even() -> Result<()> {
  assert_output_eq(
    &["generate:4:4", "even"],
    &[
      "P3 4 4 255",
      "255 255 255 255 255 255 255 255 255 255 255 255",
      "0     0   0   0   0   0   0   0   0   0   0   0",
      "255 255 255 255 255 255 255 255 255 255 255 255",
      " 0    0   0   0   0   0   0   0   0   0   0   0",
    ],
  )
}

#[test]
fn top() -> Result<()> {
  let output = Command::new(executable_path("degenerate"))
    .args(["generate:2:2", "top"])
    .output()?;

  assert!(
    output.status.success(),
    "{}",
    str::from_utf8(&output.stderr)?
  );

  assert_eq!(
    str::from_utf8(&output.stdout)?,
    "P3\n2 2 255\n255 255 255 255 255 255 0 0 0 0 0 0 "
  );

  Ok(())
}

#[test]
fn generate() -> Result<()> {
  let output = Command::new(executable_path("degenerate"))
    .args(["generate:1:1"])
    .output()?;

  assert!(
    output.status.success(),
    "{}",
    str::from_utf8(&output.stderr)?
  );

  assert_eq!(str::from_utf8(&output.stdout)?, "P3\n1 1 255\n0 0 0 ");

  Ok(())
}

#[test]
fn invert() -> Result<()> {
  assert_output_eq(&["generate:1:1", "invert"], &["P3 1 1 255", "255 255 255"]);

  Ok(())
}
