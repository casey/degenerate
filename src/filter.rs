use super::*;

pub(crate) enum Filter {
  All,
  Circle,
  Even,
  Mod { divisor: usize, remainder: usize },
  Square,
  Top,
}

impl Filter {
  pub(crate) fn filter(&self, state: &State, col: usize, row: usize) -> bool {
    match self {
      Self::Circle => {
        let width = state.matrix.ncols() as f32;
        let height = state.matrix.nrows() as f32;
        let col = col as f32 + 0.5;
        let row = row as f32 + 0.5;
        (col - (width / 2.0)).powf(2.0) + (row - (height / 2.0)).powf(2.0)
          <= (width / 2.0).powf(2.0)
      }
      Self::Even => row % 2 == 0,
      Self::All => true,
      Self::Mod { divisor, remainder } => {
        (col * state.matrix.nrows() + row) % divisor == *remainder
      }
      Self::Top => row < state.matrix.nrows() / 2,
      Self::Square => {
        let dimensions = state.dimensions();
        let (x1, y1) = (dimensions.x as f32 / 4.0, dimensions.y as f32 / 4.0);
        let (x2, y2) = (
          x1 + dimensions.x as f32 / 2.0,
          y1 + dimensions.y as f32 / 2.0,
        );
        let (row, col) = (row as f32, col as f32);
        col >= x1 && col < x2 && row >= y1 && row < y2
      }
    }
  }
}
