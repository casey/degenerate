use {
  executable_path::executable_path,
  pretty_assertions::assert_eq,
  std::{
    fs,
    io::prelude::*,
    process::{Command, Stdio},
    str,
  },
};

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn assert_output_eq(args: &str, expected_bitmap: &str) -> Result<()> {
  let mut command = Command::new(executable_path("degenerate"));

  command.args(args.split_whitespace());

  let output = command.output()?;

  assert!(
    output.status.success(),
    "Command {:?} failed: {}",
    command,
    str::from_utf8(&output.stderr)?
  );

  let mut expected_bitmap = expected_bitmap.replace(" ", "");
  if !expected_bitmap.is_empty() {
    expected_bitmap.push('\n');
  }

  assert_eq!(str::from_utf8(&output.stdout)?, expected_bitmap);

  Ok(())
}

#[test]
fn circle() -> Result<()> {
  assert_output_eq(
    "resize:10:10 circle print",
    "000FFFF000
     0FFFFFFFF0
     0FFFFFFFF0
     FFFFFFFFFF
     FFFFFFFFFF
     FFFFFFFFFF
     FFFFFFFFFF
     0FFFFFFFF0
     0FFFFFFFF0
     000FFFF000",
  )
}

#[test]
fn even() -> Result<()> {
  assert_output_eq(
    "resize:4:4 even print",
    "FFFF
     0000
     FFFF
     0000",
  )
}

#[test]
fn top() -> Result<()> {
  assert_output_eq(
    "resize:2:2 top print",
    "FF
     00",
  )
}

#[test]
fn repl_valid_filter() -> Result<()> {
  let mut command = Command::new(executable_path("degenerate"))
    .args(["resize:4:4", "repl"])
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()?;

  let stdin = command.stdin.as_mut().unwrap();
  write!(stdin, "even")?;

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
  assert_output_eq("resize:2:1 print", "00")
}

#[test]
fn invert() -> Result<()> {
  assert_output_eq("resize:1:1 all print", "F")
}

#[test]
fn save() -> Result<()> {
  assert_output_eq(
    "resize:1:2 top save:output.png print",
    "F
     0",
  )?;

  let image = image::open("output.png")?.as_rgb8().unwrap().to_owned();
  assert_eq!(image.width(), 1);
  assert_eq!(image.height(), 2);
  assert_eq!(image.to_vec(), &[255, 255, 255, 0, 0, 0]);

  Ok(())
}

#[test]
fn save_invalid_format() -> Result<()> {
  let output = Command::new(executable_path("degenerate"))
    .args(["resize:4:4", "top", "save:output.txt"])
    .output()?;

  assert!(!output.status.success());

  Ok(())
}

#[test]
fn square() -> Result<()> {
  assert_output_eq(
    "resize:4:4 square print",
    "0000
     0FF0
     0FF0
     0000",
  )
}

#[test]
fn modulus() -> Result<()> {
  assert_output_eq(
    "resize:4:2 mod:2:0 print",
    "FFFF
     0000",
  )
}

#[test]
fn default_bitmap_size() -> Result<()> {
  assert_output_eq("print", "")
}

#[ignore]
#[test]
fn default_image_size() -> Result<()> {
  let output = Command::new(executable_path("degenerate"))
    .arg("--output=output.txt")
    .output()?;

  assert!(
    output.status.success(),
    "Command failed: {}",
    str::from_utf8(&output.stderr)?
  );

  let content = fs::read_to_string("output.txt")?;
  let lines = content.lines();

  assert_eq!(lines.clone().count(), 4096);

  for line in lines {
    assert_eq!(line.len(), 4096);
  }

  Ok(())
}

#[test]
fn random() -> Result<()> {
  assert_output_eq(
    "resize:4:2 random all print",
    "8569
     3275",
  )
}

#[test]
fn reset_filter() -> Result<()> {
  assert_output_eq(
    "resize:4:2 random all invert all print",
    "7A96
     CD8A",
  )
}
