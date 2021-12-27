use super::*;

#[derive(Clone, Debug)]
pub(crate) enum Filter {
  All,
  Circle,
  Cross,
  Mod { divisor: usize, remainder: usize },
  Rows { nrows: usize, step: usize },
  Square,
  Top,
  X,
}

impl Filter {
  pub(crate) fn filter(&self, state: &State, pixel: Vector2<usize>, v: Vector2<f64>) -> bool {
    match self {
      Self::All => true,
      Self::Circle => v.norm() < 1.0,
      Self::Cross => v.x.abs() < 0.25 || v.y.abs() < 0.25,
      Self::Mod { divisor, remainder } => {
        (pixel.x * state.matrix.nrows() + pixel.y) % divisor == *remainder
      }
      Self::Rows { nrows, step } => pixel.y % (nrows + step) < *nrows,
      Self::Square => v.abs() < Vector2::new(0.5, 0.5),
      Self::Top => v.y < 0.0,
      Self::X => (v.x - v.y).abs() < 0.25 || (v.x + v.y).abs() < 0.25,
    }
  }
}
