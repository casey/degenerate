use super::*;

pub fn test(name: &str, program: &str) -> Result {
  let destination = format!("images/{}.native-actual-memory.png", name);

  fs::remove_file(&destination).ok();

  let tempdir = Test::new()?.program(program).run_and_return_tempdir()?;

  let actual_path = tempdir.path().join("memory.png");

  let actual_image = image::open(&actual_path)?;

  let expected_path = format!("images/{}.png", name);

  if !Path::new(&expected_path).is_file() || actual_image != image::open(&expected_path)? {
    fs::rename(&actual_path, &destination)?;

    #[cfg(target_os = "macos")]
    {
      let status = Command::new("xattr")
        .args(["-wx", "com.apple.FinderInfo"])
        .arg("0000000000000000000C00000000000000000000000000000000000000000000")
        .arg(&destination)
        .status()?;

      if !status.success() {
        panic!("xattr failed: {}", status);
      }
    }

    panic!(
      "Image test failed:\nExpected: {}\nActual:   {}",
      expected_path, destination,
    );
  }

  Ok(())
}

