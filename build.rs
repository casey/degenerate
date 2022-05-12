use {
  camino::Utf8PathBuf,
  indoc::indoc,
  std::{
    fs::{self, DirEntry},
    io::{self, Write},
  },
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut programs = Vec::new();

  println!("cargo:rerun-if-changed=images");

  let mut file = fs::File::create("tests/image_tests.rs")?;

  write!(file, "use super::*;")?;

  let mut entries = fs::read_dir("images")?.collect::<io::Result<Vec<DirEntry>>>()?;
  entries.sort_by_key(|entry| entry.file_name());

  for entry in entries {
    let path = Utf8PathBuf::try_from(entry.path())?;
    println!("cargo:rerun-if-changed={}", path);

    let filename = path
      .file_name()
      .ok_or_else(|| format!("Could not extract file name: {}", path))?;

    if filename.ends_with(".degen") && !path.with_extension("png").is_file() {
      panic!("Image for program `{}` missing.", filename);
    }

    if !filename.ends_with(".png") || filename.ends_with(".actual-memory.png") {
      continue;
    }

    let name = path
      .file_stem()
      .ok_or_else(|| format!("Could not extract file stem: {}", path))?
      .to_owned();

    let program = fs::read_to_string(format!("images/{}.degen", name))?
      .trim()
      .to_owned();

    programs.push((name.clone(), program.clone()));

    write!(
      file,
      indoc!(
        "


        #[test]{}
        fn {}() -> Result<()> {{
          image_test(\"{}\")
        }}",
      ),
      if program.contains("comment:slow") {
        "\n#[ignore]"
      } else {
        ""
      },
      name,
      name
    )?;
  }

  writeln!(file)?;

  Ok(())
}
