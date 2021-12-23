use super::*;

#[derive(Clone, Debug)]
pub(crate) enum Filter {
  All,
  Circle,
  Mod { divisor: usize, remainder: usize },
  Rows { limit: usize, step: usize },
  Square,
  Top,
}

impl Filter {
  pub(crate) fn filter(&self, state: &State, row: usize, col: usize) -> bool {
    match self {
      Self::All => true,
      Self::Circle => {
        let width = state.matrix.ncols() as f32;
        let height = state.matrix.nrows() as f32;
        let col = col as f32 + 0.5;
        let row = row as f32 + 0.5;
        (col - (width / 2.0)).powf(2.0) + (row - (height / 2.0)).powf(2.0)
          <= (width / 2.0).powf(2.0)
      }
      Self::Mod { divisor, remainder } => {
        (col * state.matrix.nrows() + row) % divisor == *remainder
      }
      Self::Rows { limit, step } => (0..state.matrix.nrows())
        .step_by(*step + limit)
        .map(|row| (row..=row + (*limit - 1)))
        .flatten()
        .any(|x| x == row),
      Self::Square => {
        let dimensions = state.dimensions();
        let (x1, y1) = (dimensions.1 as f32 / 4.0, dimensions.0 as f32 / 4.0);
        let (x2, y2) = (
          x1 + dimensions.1 as f32 / 2.0,
          y1 + dimensions.0 as f32 / 2.0,
        );
        let (row, col) = (row as f32, col as f32);
        col >= x1 && col < x2 && row >= y1 && row < y2
      }
      Self::Top => row < state.matrix.nrows() / 2,
    }
  }
}
