use super::*;

pub(crate) enum Filter {
  Circle,
  Even,
  Generate { width: usize, height: usize },
  Invert,
  Modulus { divisor: usize, remainder: usize },
  Pixelate { size: usize },
  Top,
}

impl Filter {
  pub(crate) fn apply(&self, state: &mut State) {
    match self {
      Self::Generate { width, height } => {
        state.generate(*width, *height);
      }
      Self::Invert => {
        state
          .scalars_mut()
          .iter_mut()
          .for_each(|scalar| *scalar = !*scalar);
      }
      Self::Modulus { divisor, remainder } => {
        state
          .scalars_mut()
          .iter_mut()
          .enumerate()
          .for_each(|(i, scalar)| {
            if i % divisor == *remainder {
              *scalar = !*scalar;
            }
          })
      }
      Self::Top => {
        let (width, height) = state.dimensions();
        state
          .scalars_mut()
          .chunks_mut(width * 3)
          .enumerate()
          .for_each(|(i, line)| {
            if i < height / 2 {
              line.iter_mut().for_each(|scalar| *scalar = !*scalar);
            }
          });
      }
      Self::Even => {
        let (width, _) = state.dimensions();
        state
          .scalars_mut()
          .chunks_mut(width * 3)
          .enumerate()
          .for_each(|(i, line)| {
            if i & 1 == 0 {
              line.iter_mut().for_each(|scalar| *scalar = !*scalar);
            }
          });
      }
      Self::Circle => {
        let (width, height) = state.dimensions();
        state
          .scalars_mut()
          .chunks_mut(width * 3)
          .enumerate()
          .for_each(|(row, line)| {
            line.iter_mut().enumerate().for_each(|(col, scalar)| {
              if (col as i64 / 3 - (width as i64 / 2)).pow(2)
                + (row as i64 - (height as i64 / 2)).pow(2)
                <= (width as i64 / 2).pow(2)
              {
                *scalar = !*scalar;
              }
            })
          });
      }
      Self::Pixelate { size } => {
        for row in 0..state.height() {
          for col in 0..state.width() {
            let source_row = row / size * size + size / 2;
            let source_col = col / size * size + size / 2;
            let source_pixel = state.get_pixel(source_row, source_col);
            state.set_pixel(row, col, source_pixel);
          }
        }
      }
    }
  }
}

impl FromStr for Filter {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.split(':').collect::<Vec<&str>>().as_slice() {
      ["circle"] => Ok(Self::Circle),
      ["even"] => Ok(Self::Even),
      ["generate", width, height] => Ok(Self::Generate {
        width: width.parse()?,
        height: height.parse()?,
      }),
      ["invert"] => Ok(Self::Invert),
      ["modulus", divisor, remainder] => Ok(Self::Modulus {
        divisor: divisor.parse()?,
        remainder: remainder.parse()?,
      }),
      ["pixelate", size] => Ok(Self::Pixelate {
        size: size.parse()?,
      }),
      ["top"] => Ok(Self::Top),
      _ => Err(format!("Invalid filter: {}", s).into()),
    }
  }
}
