use super::*;

#[derive(AsRefStr, Copy, Clone, Debug, Deserialize, EnumVariantNames, PartialEq)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum Operation {
  Debug,
  Identity,
  Invert,
  RotateColor(ColorAxis, f64),
}
