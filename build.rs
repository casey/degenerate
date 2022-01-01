use {
  camino::Utf8PathBuf,
  indoc::indoc,
  std::{
    fs::{self, File},
    io::Write,
  },
};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

fn main() -> Result {
  let mut programs = Vec::new();

  {
    println!("cargo:rerun-if-changed=images");

    let mut file = fs::File::create("tests/image_tests.rs")?;

    write!(file, "use super::*;")?;

    for result in fs::read_dir("images")? {
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

      let identifier = program.replace(|c: char| !c.is_alphanumeric(), "_");

      write!(
        file,
        indoc!(
          "


        #[test]{}
        fn {}() -> Result<()> {{
          image_test(\"{}\")
        }}",
        ),
        if program.contains("comment:ignore") {
          "\n#[ignore]"
        } else {
          ""
        },
        identifier,
        program
      )?;
    }

    writeln!(file)?;
  }

  {
    println!("cargo:rerun-if-changed=README.md");
    let text = fs::read_to_string("README.md")?;

    let mut file = File::create("README.md")?;

    for line in text.lines() {
      writeln!(file, "{}", line)?;

      if line == "## Gallery" {
        break;
      }
    }

    for program in programs {
      writeln!(file)?;
      writeln!(file, "```bash\n$ degenerate {}\n```", program)?;
      writeln!(
        file,
        "![{}](images/{}.png)",
        program,
        urlencoding::encode(&program)
      )?;
    }

    Ok(())
  }
}
