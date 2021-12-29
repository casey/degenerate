use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut programs = Vec::new();

  for result in std::fs::read_dir("images")? {
    let entry = result?;
    eprintln!("Generating image test on {}…", entry.path().display());

    let filename = entry
      .file_name()
      .into_string()
      .map_err(|filename| format!("Could not convert filename to unicode: {:?}", filename))?;

    if !filename.ends_with(".png") || filename.ends_with(".actual-output.png") {
      continue;
    }

    let expected_path = entry.path();

    let program = expected_path
      .file_stem()
      .ok_or_else(|| format!("Could not extract file stem: {}", expected_path.display()))?
      .to_str()
      .ok_or_else(|| format!("Path was not valid UTF-8: {}", expected_path.display()))?;

    programs.push(program.to_owned());
  }

  let mut file = std::fs::File::create("tests/image.rs")?;

  write!(
    file,
    "{}",
    r#"
use shared::{Test, Result, image_test};

mod shared;
"#
  )?;

  for program in programs {
    let identifier = program.replace(|c| !matches!(c, 'a'..='z' | '0'..='9'), "_");

    write!(
      file,
      r#"#[test]
fn {}() -> Result<()> {{
  image_test("{}")
}}
"#,
      identifier, program
    )?;
  }

  Ok(())
}
