use {
  executable_path::executable_path,
  image::GenericImageView,
  std::{process::Command, str},
};

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

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
