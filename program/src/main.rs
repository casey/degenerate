#![allow(unused)]

use degenerate::*;

fn fade_in(system: &System) {
  system.clear();
  system.render(Filter::new().x().alpha((system.time() / 5000.0).min(1.0)));
}

fn stretch(system: &System) {
  system.clear();
  system.render(
    Filter::new()
      .circle()
      .coordinate_transform(Scale2::new(1.0 / (system.time() / 10000.0), 2.0))
      .times(8),
  );
}

fn target(system: &System) {
  if system.frame() == 0 {
    system.clear();
    system.render(
      Filter::new()
        .circle()
        .coordinate_transform(Similarity2::from_scaling(2.0))
        .times(8),
    );
  }
}

fn kaleidoscope(system: &System) {
  let r = 5.0 / 6.0 * TAU;
  let s = 1.0 / 0.75;
  system.clear();

  system.render(
    Filter::new()
      .circle()
      .color_transform(Rotation3::from_axis_angle(&Vector3::y_axis(), 0.05 * TAU))
      .coordinate_transform(Similarity2::from_scaling(s))
      .wrap(true)
      .times(8),
  );

  let r = r + system.time() / 30000.0 * TAU;

  system.render(
    Filter::new()
      .circle()
      .color_transform(Rotation3::from_axis_angle(&Vector3::z_axis(), 0.05 * TAU))
      .coordinate_transform(Similarity2::from_parts(
        Translation2::identity(),
        Rotation2::new(r).into(),
        s,
      ))
      .wrap(true)
      .times(8),
  );
}

fn orbs(system: &System) {
  system.clear();

  system.render(
    Filter::new()
      .circle()
      .color_transform(Rotation3::from_axis_angle(&Vector3::y_axis(), 0.05 * TAU))
      .coordinate_transform(Similarity2::from_scaling(1.0 / 0.75))
      .wrap(true)
      .times(8),
  );

  system.render(
    Filter::new()
      .circle()
      .color_transform(Rotation3::from_axis_angle(&Vector3::z_axis(), 0.05 * TAU))
      .coordinate_transform(Similarity2::from_scaling(1.0 / 0.75))
      .wrap(true)
      .times(8),
  );
}

fn x(system: &System) {
  system.clear();
  for i in 0..8 {
    system.render(
      Filter::new()
        .x()
        .wrap(i % 2 == 1)
        .coordinate_transform(Similarity2::from_scaling(2.0)),
    );
  }
}

fn main() {
  System::execute(orbs);
}
