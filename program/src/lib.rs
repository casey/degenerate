use {
  degenerate::{Filter, Process, System},
  wasm_bindgen::prelude::wasm_bindgen,
};

// TODO:
// - hide ui
// - avoid UI flash
// - automatically reload when binary changes

struct Program {
  system: System,
}

impl Process for Program {
  fn new(system: System) -> Self {
    Self { system }
  }

  fn init(&mut self) {
    self.system.render(Filter::default());
  }
}

#[wasm_bindgen(start)]
pub fn start() {
  Program::execute();
}
