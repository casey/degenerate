use degenerate::{Filter, Process, System};

struct FadeIn {
  system: System,
}

impl Process for FadeIn {
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

struct X {
  system: System,
}

impl Process for X {
  fn new(system: System) -> Self {
    Self { system }
  }

  fn init(&mut self) {
    let mut filter = Filter::default().x();
    filter.coordinate_transform = [2.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 1.0];

    for i in 0..8 {
      self.system.render(filter);
      filter.wrap = i % 2 == 0;
    }
  }
}

fn main() {
  // FadeIn::execute();
  X::execute();
}
