use super::*;

pub(crate) enum Filter {
  Generate { width: usize, height: usize },
  Invert,
  Modulus { divisor: usize, remainder: usize },
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
      _ => Err(format!("Invalid filter: {}", s).into()),
    }
  }
}
