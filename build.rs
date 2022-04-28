use {
  camino::Utf8PathBuf,
  indoc::indoc,
  std::{
    fs::{self, DirEntry, File},
    io::{self, Write},
  },
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut programs = Vec::new();

  {
    println!("cargo:rerun-if-changed=images");

    let mut file = fs::File::create("tests/image_tests.rs")?;

    write!(file, "use super::*;")?;

    let mut entries = fs::read_dir("images")?.collect::<io::Result<Vec<DirEntry>>>()?;
    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
      let expected_path = Utf8PathBuf::try_from(entry.path())?;
      println!("cargo:rerun-if-changed={}", expected_path);

      let filename = expected_path
        .file_name()
        .ok_or_else(|| format!("Could not extract file name: {}", expected_path))?;

      if !filename.ends_with(".png") || filename.ends_with(".actual-memory.png") {
        continue;
      }

      let filestem = expected_path
        .file_stem()
        .ok_or_else(|| format!("Could not extract file stem: {}", expected_path))?;

      let program = fs::read_to_string(format!("images/{}.degen", filestem))?;

      programs.push(program.to_owned());

      let identifier = format!("test_{}", filestem);

      write!(
        file,
        indoc!(
          "


        #[test]{}
        #[rustfmt::skip]
        fn {}() -> Result<()> {{
          image_test(\"{}\")
        }}",
        ),
        if program.contains("comment:slow") {
          "\n#[ignore]"
        } else {
          ""
        },
        identifier,
        filestem
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
      writeln!(file, "```\n$ degenerate {}\n```", program)?;
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
