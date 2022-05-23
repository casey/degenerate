use super::*;

#[derive(Debug, Copy, Clone, EnumString, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub(crate) enum ColorAxis {
  #[strum(serialize = "r", serialize = "red")]
  Red,
  #[strum(serialize = "g", serialize = "green")]
  Green,
  #[strum(serialize = "b", serialize = "blue")]
  Blue,
}

impl Default for ColorAxis {
  fn default() -> Self {
    ColorAxis::Red
  }
}

impl ColorAxis {
  pub(crate) fn vector(&self) -> Vector3<f64> {
    match self {
      ColorAxis::Red => Vector3::x(),
      ColorAxis::Green => Vector3::y(),
      ColorAxis::Blue => Vector3::z(),
    }
  }
}
