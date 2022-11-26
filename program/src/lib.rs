use {
  degenerate::{execute, post_message, AppMessage, WorkerMessage},
  wasm_bindgen::prelude::wasm_bindgen,
};

struct Program {}

impl degenerate::Program for Program {
  fn new() -> Self {
    Self {}
  }

  fn on_message(&mut self, message: AppMessage) {
    if let AppMessage::Frame = message {
      post_message(WorkerMessage::Save);
    }
  }
}

#[wasm_bindgen(start)]
pub fn start() {
  execute::<Program>();
}
