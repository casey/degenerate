use {
  degenerate::{Process, System},
  wasm_bindgen::prelude::wasm_bindgen,
};

struct Program {
  system: System,
}

impl Process for Program {
  fn new(system: System) -> Self {
    Self { system }
  }

  fn on_frame(&mut self) {
    self.system.save()
  }
}

#[wasm_bindgen(start)]
pub fn start() {
  Program::execute();
}
