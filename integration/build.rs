use std::process::Command;

fn main() {
  println!("cargo:rerun-if-changed=..");

  eprintln!("Building WASM binary...");

  let status = Command::new("cargo")
    .args(["build", "--release", "--target", "wasm32-unknown-unknown"])
    .current_dir("..")
    .status()
    .unwrap();

  if !status.success() {
    panic!("Failed to build WASM binary: {status}");
  }

  eprintln!("Running wasm-bindgen...");

  let status = Command::new("wasm-bindgen")
    .args([
      "--target",
      "web",
      "--no-typescript",
      "target/wasm32-unknown-unknown/release/degenerate.wasm",
      "--out-dir",
      "integration/www",
    ])
    .current_dir("..")
    .status()
    .unwrap();

  if !status.success() {
    panic!("wasm-bindgen failed: {status}");
  }

  eprintln!("Done with setup!");
}
