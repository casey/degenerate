use super::*;

pub(crate) enum Filter {
  All,
  Circle,
  Even,
  Mod { divisor: usize, remainder: usize },
  Repl,
  Resize { rows: usize, cols: usize },
  Save { path: PathBuf },
  Square,
  Top,
}

impl Filter {
  pub(crate) fn apply(&self, state: &mut State) -> Result<()> {
    match self {
      Self::Circle => {
        let (width, height) = (state.dimensions().x as f32, state.dimensions().y as f32);
        state
          .matrix()
          .row_iter_mut()
          .enumerate()
          .for_each(|(row, mut line)| {
            line.iter_mut().enumerate().for_each(|(col, pixel)| {
              let (row, col) = (row as f32 + 0.5, col as f32 + 0.5);
              if (col - (width / 2.0)).powf(2.0) + (row - (height / 2.0)).powf(2.0)
                <= (width / 2.0).powf(2.0)
              {
                pixel.iter_mut().for_each(|scalar| *scalar = !*scalar);
              }
            })
          });
      }
      Self::Even => {
        let height = state.dimensions().y;
        state
          .matrix()
          .rows_with_step_mut(0, height / 2, 1)
          .iter_mut()
          .for_each(|row| {
            row.iter_mut().for_each(|scalar| *scalar = !*scalar);
          });
      }
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
      Self::Save { path } => match path.extension() {
        Some(ext) if ext == "txt" => {
          panic!(
            "Cannot save state to {}: Invalid file format `.txt`.",
            path.display()
          );
        }
        _ => state.save(path)?,
      },
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
      ["all"] => Ok(Self::All),
      ["circle"] => Ok(Self::Circle),
      ["even"] => Ok(Self::Even),
      ["mod", divisor, remainder] => Ok(Self::Mod {
        divisor: divisor.parse()?,
        remainder: remainder.parse()?,
      }),
      ["repl"] => Ok(Self::Repl),
      ["resize", cols, rows] => Ok(Self::Resize {
        cols: cols.parse()?,
        rows: rows.parse()?,
      }),
      ["save", path] => Ok(Self::Save {
        path: path.parse()?,
      }),
      ["square"] => Ok(Self::Square),
      ["top"] => Ok(Self::Top),
      _ => Err(format!("Invalid filter: {}", s).into()),
    }
  }
}
