use super::*;

pub(crate) enum Filter {
  // Circle,
  Even,
  Generate { width: usize, height: usize },
  Invert,
  // Modulus { divisor: usize, remainder: usize },
  // Pixelate { size: usize },
  Top,
  // Square,
}

impl Filter {
  pub(crate) fn apply(&self, state: &mut State) {
    match self {
      // Self::Circle => {
      //   let (width, height) = state.dimensions();
      //   let (width, height) = (width as f32, height as f32);
      //   state.rows_mut().enumerate().for_each(|(row, line)| {
      //     line.iter_mut().enumerate().for_each(|(col, scalar)| {
      //       let (row, col) = (row as f32, col as f32);
      //       if (col / 3.0 - (width / 2.0)).powf(2.0) + (row - (height / 2.0)).powf(2.0)
      //         <= (width / 2.0).powf(2.0)
      //       {
      //         *scalar = !*scalar;
      //       }
      //     })
      //   });
      // }
      Self::Even => {
        let height = state.height();
        state
          .matrix()
          .rows_with_step_mut(0, height / 2, 1)
          .iter_mut()
          .for_each(|row| {
            row.iter_mut().for_each(|scalar| *scalar = !*scalar);
          });
      }
      Self::Generate { width, height } => {
        state.generate(*width, *height);
      }
      Self::Invert => {
        state
          .matrix()
          .iter_mut()
          .for_each(|pixel| pixel.iter_mut().for_each(|scalar| *scalar = !*scalar));
      } // Self::Modulus { divisor, remainder } => {
      //   state
      //     .scalars_mut()
      //     .iter_mut()
      //     .enumerate()
      //     .for_each(|(i, scalar)| {
      //       if i % divisor == *remainder {
      //         *scalar = !*scalar;
      //       }
      //     })
      // }
      // Self::Square => {
      //   let (width, height) = state.dimensions();
      //   let (x1, y1) = (width / 4, height / 4);
      //   let (x2, y2) = (x1 + width / 2, y1 + height / 2);
      //   state.rows_mut().enumerate().for_each(|(row, line)| {
      //     line.iter_mut().enumerate().for_each(|(col, scalar)| {
      //       if col / 3 > x1 && col / 3 < x2 && row > y1 && row < y2 {
      //         *scalar = !*scalar;
      //       }
      //     })
      //   });
      // }
      // Self::Pixelate { size } => {
      //   for row in 0..state.height() {
      //     for col in 0..state.width() {
      //       let source_row = row / size * size + size / 2;
      //       let source_col = col / size * size + size / 2;
      //       let source_pixel = state.get_pixel(source_row, source_col);
      //       state.set_pixel(row, col, source_pixel);
      //     }
      //   }
      // }
      Self::Top => {
        let height = state.height();
        state
          .matrix()
          .rows_mut(0, height / 2)
          .iter_mut()
          .for_each(|row| row.iter_mut().for_each(|scalar| *scalar = !*scalar));
      }
    }
  }
}

impl FromStr for Filter {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.split(':').collect::<Vec<&str>>().as_slice() {
      // ["circle"] => Ok(Self::Circle),
      // ["square"] => Ok(Self::Square),
      ["even"] => Ok(Self::Even),
      ["generate", width, height] => Ok(Self::Generate {
        width: width.parse()?,
        height: height.parse()?,
      }),
      ["invert"] => Ok(Self::Invert),
      // ["modulus", divisor, remainder] => Ok(Self::Modulus {
      //   divisor: divisor.parse()?,
      //   remainder: remainder.parse()?,
      // }),
      // ["pixelate", size] => Ok(Self::Pixelate {
      //   size: size.parse()?,
      // }),
      ["top"] => Ok(Self::Top),
      _ => Err(format!("Invalid filter: {}", s).into()),
    }
  }
}
