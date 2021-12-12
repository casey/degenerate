use super::*;

pub(crate) enum Filter {
  Generate { width: usize, height: usize },
  Invert,
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
      ["top"] => Ok(Self::Top),
      _ => Err(format!("Invalid filter: {}", s).into()),
    }
  }
}
