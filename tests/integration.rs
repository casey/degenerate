use {
  executable_path::executable_path,
  std::{process::Command, str},
};

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

#[test]
fn even() -> Result<()> {
  let output = Command::new(executable_path("degenerate"))
    .args(["generate:4:4", "even"])
    .output()?;

  assert!(
    output.status.success(),
    "{}",
    str::from_utf8(&output.stderr)?
  );

  assert_eq!(
    str::from_utf8(&output.stdout)?,
    "P3\n4 4 255\n255 255 255 255 255 255 255 255 255 255 255 255 0 0 0 0 0 0 0 0 0 0 0 \n0 255 255 255 255 255 255 255 255 255 255 255 255 0 0 0 0 0 0 0 0 0 0 \n0 0 "
  );

  Ok(())
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
  let output = Command::new(executable_path("degenerate"))
    .args(["generate:1:1", "invert"])
    .output()?;

  assert!(
    output.status.success(),
    "{}",
    str::from_utf8(&output.stderr)?
  );

  assert_eq!(str::from_utf8(&output.stdout)?, "P3\n1 1 255\n255 255 255 ");

  Ok(())
}
