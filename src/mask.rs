use {super::*, rand_derive2::RandGen};

#[derive(Clone, Debug, RandGen, PartialEq)]
pub(crate) enum Mask {
  All,
  Circle,
  Cross,
  Mod { divisor: u64, remainder: u64 },
  Rows { nrows: u64, step: u64 },
  Square,
  Top,
  X,
}

use std::fmt::{self, Display, Formatter};

impl Display for Mask {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Circle => "circle",
        Self::Cross => "cross",
        Self::Mod { .. } => "mod",
        Self::Rows { .. } => "rows",
        Self::Square => "square",
        Self::Top => "top",
        Self::X => "x",
        Self::All => "all",
      }
    )
  }
}

impl Mask {
  pub(crate) fn is_masked(
    &self,
    dimensions: Vector2<usize>,
    pixel: Point2<isize>,
    v: Point2<f64>,
  ) -> bool {
    match self {
      Self::All => true,
      Self::Circle => v.coords.norm() < 1.0,
      Self::Cross => v.x.abs() < 0.25 || v.y.abs() < 0.25,
      Self::Mod { divisor, remainder } => {
        if *divisor == 0 {
          false
        } else {
          ((pixel.x as u64)
            .wrapping_mul(dimensions.y as u64)
            .wrapping_add(pixel.y as u64))
            % *divisor
            == *remainder
        }
      }
      Self::Rows { nrows, step } => pixel.y as u64 % (nrows.saturating_add(*step)) < *nrows,
      Self::Square => v.coords.abs() < Vector2::new(0.5, 0.5),
      Self::Top => v.y < 0.0,
      Self::X => (v.x - v.y).abs() < 0.25 || (v.x + v.y).abs() < 0.25,
    }
  }
}
