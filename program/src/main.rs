#![allow(unused_variables, dead_code)]

use degenerate::*;

fn fade_in(frame: Frame) {
  Filter::new()
    .x()
    .alpha((frame.time / 5000.0).min(1.0))
    .render();
}

fn stretch(frame: Frame) {
  Filter::new()
    .circle()
    .position(Scale2::new(1.0 / (frame.time / 10000.0), 2.0))
    .times(8)
    .render();
}

fn target(frame: Frame) {
  Filter::new()
    .circle()
    .position(Similarity2::from_scaling(2.0))
    .times(8)
    .render();
}

fn kaleidoscope(frame: Frame) {
  let s = 1.0 / 0.75;
  Filter::new()
    .circle()
    .color(Rotation3::from_axis_angle(&Vector3::y_axis(), 0.05 * TAU))
    .position(Similarity2::from_scaling(s))
    .wrap(true)
    .times(8)
    .render()
    .color(Rotation3::from_axis_angle(&Vector3::z_axis(), 0.05 * TAU))
    .position(Similarity2::from_parts(
      Translation2::identity(),
      Rotation2::new(5.0 / 6.0 * TAU + frame.time / 30000.0 * TAU).into(),
      s,
    ))
    .render();
}

fn orbs(frame: Frame) {
  Filter::new()
    .circle()
    .color(Rotation3::from_axis_angle(&Vector3::y_axis(), 0.05 * TAU))
    .position(Similarity2::from_scaling(1.0 / 0.75))
    .wrap(true)
    .times(8)
    .render()
    .color(Rotation3::from_axis_angle(&Vector3::z_axis(), 0.05 * TAU))
    .render();
}

fn x(frame: Frame) {
  for i in 0..16 {
    Filter::new()
      .x()
      .wrap(true)
      .position(Similarity2::from_scaling(2.0))
      .render();
  }
}

fn frequency(frame: Frame) {
  Filter::new().frequency().render();
}

fn equalizer(frame: Frame) {
  Filter::new().equalizer().render();
}

fn pattern(frame: Frame) {
  for i in 0..8 {
    Filter::new()
      .alpha(0.75)
      .circle()
      .position(Similarity2::from_scaling(2.0))
      .wrap(i % 2 == 0)
      .render();
  }
}

fn pattern_opaque(frame: Frame) {
  for i in 0..8 {
    Filter::new()
      .circle()
      .position(Similarity2::from_scaling(2.0))
      .wrap(i % 2 == 0)
      .render();
  }
}

fn main() {
  send(Message::Record);
  x.execute();
}
