use {camino::Utf8PathBuf, indoc::indoc, std::io::Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("cargo:rerun-if-changed=images");

  let mut programs = Vec::new();

  for result in std::fs::read_dir("images")? {
    let entry = result?;
    let expected_path = Utf8PathBuf::try_from(entry.path())?;
    println!("cargo:rerun-if-changed={}", expected_path);

    let filename = expected_path
      .file_name()
      .ok_or_else(|| format!("Could not extract file name: {}", expected_path))?;

    if !filename.ends_with(".png") || filename.ends_with(".actual-output.png") {
      continue;
    }

    let program = expected_path
      .file_stem()
      .ok_or_else(|| format!("Could not extract file stem: {}", expected_path))?;

    programs.push(program.to_owned());
  }

  let mut file = std::fs::File::create("tests/image_tests.rs")?;

  write!(file, "use super::*;")?;

  for program in programs {
    let identifier = program.replace(|c| !matches!(c, 'a'..='z' | '0'..='9'), "_");

    write!(
      file,
      indoc!(
        "


        #[test]
        fn {}() -> Result<()> {{
          image_test(\"{}\")
        }}",
      ),
      identifier, program
    )?;
  }

  writeln!(file)?;

  Ok(())
}
