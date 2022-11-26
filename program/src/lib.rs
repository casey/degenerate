use {
  degenerate::{Filter, Process, System},
  wasm_bindgen::prelude::wasm_bindgen,
};

// TODO:
// - automatically reload when binary changes
// - write multiple separate binaries
// - listen on port 80

struct Program {
  system: System,
}

impl Process for Program {
  fn new(system: System) -> Self {
    Self { system }
  }

  fn init(&mut self) {
    self.system.render(Filter::default().x());
  }
}

#[wasm_bindgen(start)]
pub fn start() {
  Program::execute();
}
