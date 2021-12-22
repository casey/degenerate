use super::*;

pub(crate) enum Filter {
  All,
  Circle,
  Even,
  Mod { divisor: usize, remainder: usize },
  Repl,
  Resize { rows: usize, cols: usize },
  Square,
  Top,
}

impl Filter {
  fn filter(&self, state: &State, col: usize, row: usize) -> bool {
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
      _ => todo!(),
    }
  }

  pub(crate) fn apply_filter(&self, state: &mut State) {
    for col in 0..state.matrix.ncols() {
      for row in 0..state.matrix.nrows() {
        if self.filter(state, col, row) {
          state.matrix[(row, col)] = state.matrix[(row, col)].map(|scalar| !scalar);
        }
      }
    }
  }

  pub(crate) fn apply(&self, state: &mut State) -> Result<()> {
    match self {
      Self::Circle => self.apply_filter(state),
      Self::Even => self.apply_filter(state),

      Self::Resize { rows, cols } => {
        state.resize(Vector2::new(*rows, *cols));
      }
      Self::Repl => {
        for result in BufReader::new(io::stdin()).lines() {
          let line = result?;
          match line.trim().parse::<Filter>() {
            Ok(filter) => {
              filter.apply(state)?;
              state.write(io::stderr())?;
            }
            Err(err) => {
              eprintln!("Could not parse filter from `{}`: {}", line, err);
            }
          }
        }
      }
      Self::All => {
        state
          .matrix()
          .iter_mut()
          .for_each(|pixel| pixel.iter_mut().for_each(|scalar| *scalar = !*scalar));
      }
      Self::Mod { divisor, remainder } => {
        state
          .matrix()
          .iter_mut()
          .enumerate()
          .for_each(|(i, pixel)| {
            if i % divisor == *remainder {
              pixel.iter_mut().for_each(|scalar| *scalar = !*scalar);
            }
          })
      }
      Self::Square => {
        let dimensions = state.dimensions();
        let (x1, y1) = (dimensions.x as f32 / 4.0, dimensions.y as f32 / 4.0);
        let (x2, y2) = (
          x1 + dimensions.x as f32 / 2.0,
          y1 + dimensions.y as f32 / 2.0,
        );
        state
          .matrix()
          .row_iter_mut()
          .enumerate()
          .for_each(|(row, mut line)| {
            line.iter_mut().enumerate().for_each(|(col, pixel)| {
              let (row, col) = (row as f32, col as f32);
              if col >= x1 && col < x2 && row >= y1 && row < y2 {
                pixel.iter_mut().for_each(|scalar| *scalar = !*scalar);
              }
            })
          });
      }
      Self::Top => {
        let height = state.dimensions().y;
        state
          .matrix()
          .rows_mut(0, height / 2)
          .iter_mut()
          .for_each(|row| row.iter_mut().for_each(|scalar| *scalar = !*scalar));
      }
    }

    Ok(())
  }
}

impl FromStr for Filter {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.split(':').collect::<Vec<&str>>().as_slice() {
      ["repl"] => Ok(Self::Repl),
      ["circle"] => Ok(Self::Circle),
      ["square"] => Ok(Self::Square),
      ["even"] => Ok(Self::Even),
      ["resize", cols, rows] => Ok(Self::Resize {
        cols: cols.parse()?,
        rows: rows.parse()?,
      }),
      ["all"] => Ok(Self::All),
      ["mod", divisor, remainder] => Ok(Self::Mod {
        divisor: divisor.parse()?,
        remainder: remainder.parse()?,
      }),
      ["top"] => Ok(Self::Top),
      _ => Err(format!("Invalid filter: {}", s).into()),
    }
  }
}
