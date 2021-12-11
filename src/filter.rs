use super::*;

pub(crate) enum Filter {
  Generate { width: usize, height: usize },
  Invert,
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
      _ => Err(format!("Invalid filter: {}", s).into()),
    }
  }
}
