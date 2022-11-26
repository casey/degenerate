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
    system.render(Filter::default());
    Self
  }
}

#[wasm_bindgen(start)]
pub fn start() {
  Program::execute();
}
