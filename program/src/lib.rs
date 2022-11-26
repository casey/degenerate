use {
  degenerate::{Filter, Process, System},
  wasm_bindgen::prelude::wasm_bindgen,
};

// TODO:
// - hide ui
// - avoid UI flash
// - automatically reload when binary changes

struct Program;

impl Process for Program {
  fn new(system: System) -> Self {
    Self { system }
  }

  fn frame(&mut self, n: u64) {
    if n == 0 {
      self.system.render(Filter::default());
    }
  }
}

#[wasm_bindgen(start)]
pub fn start() {
  Program::execute();
}
