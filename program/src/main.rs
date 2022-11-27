use degenerate::{nalgebra::Matrix3, Event, Field, Filter, Message, System};

// let r = 5 / 6 * TAU;
// let s = 1 / 0.75;

// while(true) {
//   reboot();

//   rotateColor('green', 0.05 * TAU);

//   circle();

//   scale(s);

//   wrap(!filter.wrap);

//   for (let i = 0; i < 8; i++) {
//     render();
//   }

//   if (checkbox('rotate')) {
//     r += delta() / 30000 * TAU;
//   }

//   transform(r, [s, s], [0, 0]);

//   rotateColor('blue', 0.05 * TAU);

//   for (let i = 0; i < 8; i++) {
//     render();
//   }

//   await frame();
// }

// Press the `Run` button or `Shift + Enter` to execute

fn fade_in(system: &System, event: Event) {
  if let Event::Frame(timestamp) = event {
    system.send(Message::Clear);
    system.send(Message::Render(Filter {
      field: Field::X,
      alpha: (timestamp / 5000.0).min(1.0),
      ..Filter::default()
    }));
  }
}

fn stretch(system: &System, event: &Event) {
  if let Event::Frame(t) = event {
    if system.frame() >= 0 {
      system.send(Message::Clear);
      for _ in 0..8 {
        system.send(Message::Render(Filter {
          field: Field::Circle,
          coordinate_transform: Matrix3::new(
            1.0 / (t / 1000.0),
            0.0,
            0.0,
            0.0,
            2.0,
            0.0,
            0.0,
            0.0,
            1.0,
          ),
          ..Filter::default()
        }));
      }
    }
  }
}

fn target(system: &System, event: &Event) {
  if let Event::Frame(t) = event {
    if system.frame() >= 0 {
      system.send(Message::Clear);
      for _ in 0..8 {
        system.send(Message::Render(Filter {
          field: Field::Circle,
          coordinate_transform: Matrix3::new(2.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 1.0),
          ..Filter::default()
        }));
      }
    }
  }
}

fn main() {
  System::execute(target);
}
