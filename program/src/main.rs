#![allow(unused)]

use degenerate::*;

fn fade_in(system: &System) {
  system.send(Message::Clear);
  system.render(Filter {
    field: Field::X,
    alpha: (system.time() / 5000.0).min(1.0),
    ..Filter::default()
  });
}

fn stretch(system: &System) {
  system.send(Message::Clear);
  for _ in 0..8 {
    system.render(Filter {
      field: Field::Circle,
      coordinate_transform: Scale2::new(1.0 / (system.time() / 10000.0), 2.0).into(),
      ..Filter::default()
    });
  }
}

fn target(system: &System) {
  if system.frame() == 0 {
    system.send(Message::Clear);
    for _ in 0..8 {
      system.render(Filter {
        field: Field::Circle,
        coordinate_transform: Similarity2::from_scaling(2.0).into(),
        ..Filter::default()
      });
    }
  }
}

fn kaleidoscope(system: &System) {
  let r = 5.0 / 6.0 * TAU;
  let s = 1.0 / 0.75;
  system.send(Message::Clear);

  for _ in 0..8 {
    system.render(Filter {
      field: Field::Circle,
      color_transform: Rotation3::from_axis_angle(&Vector3::y_axis(), 0.05 * TAU).into(),
      coordinate_transform: Similarity2::from_scaling(s).into(),
      wrap: true,
      ..Filter::default()
    });
  }

  let r = r + system.time() / 30000.0 * TAU;

  for _ in 0..8 {
    system.render(Filter {
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
    });
  }
}

fn orbs(system: &System) {
  system.send(Message::Clear);

  for _ in 0..8 {
    system.render(Filter {
      field: Field::Circle,
      color_transform: Rotation3::from_axis_angle(&Vector3::y_axis(), 0.05 * TAU).into(),
      coordinate_transform: Similarity2::from_scaling(1.0 / 0.75).into(),
      wrap: true,
      ..Filter::default()
    });
  }

  for _ in 0..8 {
    system.render(Filter {
      field: Field::Circle,
      color_transform: Rotation3::from_axis_angle(&Vector3::z_axis(), 0.05 * TAU).into(),
      coordinate_transform: Similarity2::from_scaling(1.0 / 0.75).into(),
      wrap: true,
      ..Filter::default()
    });
  }
}

fn x(system: &System) {
  system.send(Message::Clear);
  for i in 0..8 {
    system.render(Filter {
      field: Field::X,
      wrap: i % 2 == 1,
      coordinate_transform: Similarity2::from_scaling(2.0).into(),
      ..Filter::default()
    });
  }
}

fn main() {
  System::execute(x);
}
