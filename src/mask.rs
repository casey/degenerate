use {super::*, rand_derive2::RandGen};

#[derive(Clone, Debug, RandGen)]
pub(crate) enum Mask {
  All,
  Circle,
  Cross,
  Mod { divisor: usize, remainder: usize },
  Rows { nrows: usize, step: usize },
  Square,
  Top,
  X,
}

impl Mask {
  pub(crate) fn is_masked(&self, state: &State, pixel: Vector2<isize>, v: Vector2<f64>) -> bool {
    match self {
      Self::All => true,
      Self::Circle => v.norm() < 1.0,
      Self::Cross => v.x.abs() < 0.25 || v.y.abs() < 0.25,
      Self::Mod { divisor, remainder } => {
        ((pixel.x as usize)
          .wrapping_mul(state.matrix.nrows())
          .wrapping_add(pixel.y as usize))
          % *divisor
          == *remainder
      }
      Self::Rows { nrows, step } => pixel.y as usize % (nrows.saturating_add(*step)) < *nrows,
      Self::Square => v.abs() < Vector2::new(0.5, 0.5),
      Self::Top => v.y < 0.0,
      Self::X => (v.x - v.y).abs() < 0.25 || (v.x + v.y).abs() < 0.25,
    }
  }
}
