#![allow(unused)]

use degenerate::*;

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
    system.send(Message::Clear);
    for _ in 0..8 {
      system.send(Message::Render(Filter {
        field: Field::Circle,
        coordinate_transform: Scale2::new(1.0 / (t / 10000.0), 2.0).into(),
        ..Filter::default()
      }));
    }
  }
}

fn target(system: &System, event: &Event) {
  if let Event::Frame(_) = event {
    if system.frame() == 0 {
      system.send(Message::Clear);
      for _ in 0..8 {
        system.send(Message::Render(Filter {
          field: Field::Circle,
          coordinate_transform: Similarity2::from_scaling(2.0).into(),
          ..Filter::default()
        }));
      }
    }
  }
}

fn kaleidoscope(system: &System, event: &Event) {
  let r = 5.0 / 6.0 * TAU;
  let s = 1.0 / 0.75;
  if let Event::Frame(t) = event {
    system.send(Message::Clear);

    for _ in 0..8 {
      system.send(Message::Render(Filter {
        field: Field::Circle,
        color_transform: Rotation3::from_axis_angle(&Vector3::y_axis(), 0.05 * TAU).into(),
        coordinate_transform: Similarity2::from_scaling(s).into(),
        wrap: true,
        ..Filter::default()
      }));
    }

    let r = r + t / 30000.0 * TAU;

    for _ in 0..8 {
      system.send(Message::Render(Filter {
        field: Field::Circle,
        color_transform: Rotation3::from_axis_angle(&Vector3::z_axis(), 0.05 * TAU).into(),
        coordinate_transform: Similarity2::from_parts(
          Translation2::identity(),
          Rotation2::new(r).into(),
          s,
        )
        .into(),
        wrap: true,
        ..Filter::default()
      }));
    }
  }
}

fn orbs(system: &System, event: &Event) {
  if let Event::Frame(_) = event {
    system.send(Message::Clear);

    for _ in 0..8 {
      system.send(Message::Render(Filter {
        field: Field::Circle,
        color_transform: Rotation3::from_axis_angle(&Vector3::y_axis(), 0.05 * TAU).into(),
        coordinate_transform: Similarity2::from_scaling(1.0 / 0.75).into(),
        wrap: true,
        ..Filter::default()
      }));
    }

    for _ in 0..8 {
      system.send(Message::Render(Filter {
        field: Field::Circle,
        color_transform: Rotation3::from_axis_angle(&Vector3::z_axis(), 0.05 * TAU).into(),
        coordinate_transform: Similarity2::from_scaling(1.0 / 0.75).into(),
        wrap: true,
        ..Filter::default()
      }));
    }
  }
}

fn x(system: &System, event: &Event) {
  if let Event::Frame(_) = event {
    system.send(Message::Clear);
    for i in 0..8 {
      system.send(Message::Render(Filter {
        field: Field::X,
        wrap: i % 2 == 1,
        coordinate_transform: Similarity2::from_scaling(2.0).into(),
        ..Filter::default()
      }));
    }
  }
}

fn main() {
  System::execute(x);
}
