use super::*;

#[derive(Copy, Clone, Debug)]
pub(crate) enum Operation {
  Invert,
  RotateColor(ColorAxis, f64),
}

impl Operation {
  pub(crate) fn apply(self, element: Vector3<u8>) -> Vector3<u8> {
    match self {
      Self::Invert => element.map(|scalar| !scalar),
      Self::RotateColor(axis, turns) => {
        let v = Rotation3::new(axis.vector() * turns * f64::consts::TAU)
          * element.map(|scalar| scalar as f64 / 255.0 * 2.0 - 1.0);
        v.map(|scalar| ((scalar + 1.0) / 2.0 * 255.0).clamp(0.0, 255.0) as u8)
      }
    }
  }
}
