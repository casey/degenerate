use super::*;

#[derive(AsRefStr, Clone, Debug, Deserialize, EnumVariantNames, PartialEq)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum Mask {
  All,
  Circle,
  Cross,
  Mod { divisor: u32, remainder: u32 },
  Rows { nrows: u32, step: u32 },
  Square,
  Top,
  X,
}
