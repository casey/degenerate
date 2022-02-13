use {
  camino::Utf8PathBuf,
  cradle::prelude::*,
  std::{
    env,
    fs::{self, DirEntry},
    io,
    path::Path,
  },
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut entries = fs::read_dir("images")?.collect::<io::Result<Vec<DirEntry>>>()?;

  entries.sort_by_key(|entry| entry.file_name());

  run!(%"cargo build --release");

  let mut programs = Vec::new();

  for entry in entries {
    let expected_path = Utf8PathBuf::try_from(entry.path())?;

    let filename = expected_path
      .file_name()
      .ok_or_else(|| format!("Could not extract file name: {}", expected_path))?;

    if !filename.ends_with(".png") || filename.ends_with(".actual-output.png") {
      continue;
    }

    let mut program = expected_path
      .file_stem()
      .ok_or_else(|| format!("Could not extract file stem: {}", expected_path))?
      .split_whitespace()
      .map(str::to_owned)
      .collect::<Vec<String>>();

    if !program.iter().any(|command| command.starts_with("resize")) {
      program.insert(0, "resize:4096".into())
    }

    programs.push(program);
  }

  let gallery = Path::new("gallery");

  if gallery.is_dir() {
    fs::remove_dir_all(&gallery)?;
  }
  fs::create_dir(&gallery)?;

  let bin = env::current_dir()?.join("target/release/degenerate");
  for (i, program) in programs.iter().enumerate() {
    let tempdir = tempfile::tempdir()?;
    fs::write(tempdir.path().join("program.degen"), "x apply")?;
    eprintln!("+ {}", program.join(" "));
    run!(&bin, program, CurrentDir(tempdir.path()),);
    fs::rename(
      tempdir.path().join("output.png"),
      gallery.join(format!("{i}.png")),
    )?;
  }

  let mut index = String::new();

  index.push_str("<style>img { width: 100%; }</style>");

  for i in 0..programs.len() {
    index.push_str(&format!("<img src=\"{i}.png\">\n"));
  }

  fs::write(gallery.join("index.html"), index)?;

  Ok(())
}
