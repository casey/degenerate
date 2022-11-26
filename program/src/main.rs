use degenerate::{Filter, Process, System};

// TODO:
// - automatically reload when binary changes
// - write multiple separate binaries
// - re-render when window changes size
// - reimplement examples
// - deploy programs to web

struct Program {
  system: System,
}

impl Process for Program {
  fn new(system: System) -> Self {
    Self { system }
  }

  fn frame(&mut self, timestamp: f64) {
    self.system.clear();
    let mut filter = Filter::default().x();
    filter.alpha = (timestamp / 5000.0).min(1.0) as f32;
    self.system.render(filter);
  }
}

fn main() {
  Program::execute();
}
