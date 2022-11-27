use degenerate::{Field, Filter, Process, System};

struct Program {
  system: System,
}

impl Process for Program {
  fn new(system: System) -> Self {
    Self { system }
  }

  fn frame(&mut self, timestamp: f32) {
    self.system.clear();
    self.system.render(Filter {
      field: Field::X,
      alpha: (timestamp / 5000.0).min(1.0),
      ..Filter::default()
    });
  }
}

fn main() {
  Program::execute();
}
