use super::*;

pub(crate) enum Filter {
  Generate { width: usize, height: usize },
  Invert,
  Modulus { divisor: usize, remainder: usize },
  Top,
  Pixelate { size: usize },
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
      Self::Pixelate { size } => {
        for row in 0..state.height() {
          for col in 0..state.width() {
            let source_row = row / size * size;
            let source_col = col / size * size;
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
