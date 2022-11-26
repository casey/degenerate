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

  fn init(&mut self) {
    self.system.render(Filter::default().x());
  }
}

fn main() {
  Program::execute();
}
