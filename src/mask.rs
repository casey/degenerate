use super::*;

#[derive(AsRefStr, Clone, Debug, EnumVariantNames, PartialEq)]
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

impl Mask {
  pub(crate) fn is_masked(&self, size: usize, pixel: Point2<isize>, v: Point2<f64>) -> bool {
    match self {
      Self::All => true,
      Self::Circle => v.coords.norm() < 1.0,
      Self::Cross => v.x.abs() < 0.25 || v.y.abs() < 0.25,
      Self::Mod { divisor, remainder } => {
        if *divisor == 0 {
          false
        } else {
          ((pixel.y as u32)
            .wrapping_mul(size as u32)
            .wrapping_add(pixel.x as u32))
            % *divisor
            == *remainder
        }
      }
      Self::Rows { nrows, step } => pixel.y as u32 % (nrows.saturating_add(*step)) < *nrows,
      Self::Square => v.coords.abs() < Vector2::new(0.5, 0.5),
      Self::Top => v.y < 0.0,
      Self::X => (v.x - v.y).abs() < 0.25 || (v.x + v.y).abs() < 0.25,
    }
  }
}
