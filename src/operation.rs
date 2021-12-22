use super::*;

#[derive(Copy, Clone)]
pub(crate) enum Operation {
  Random,
  Invert,
}

impl Operation {
  pub(crate) fn apply(self, state: &mut State, element: Vector3<u8>) -> Vector3<u8> {
    match self {
      Self::Invert => element.map(|scalar| !scalar),
      Self::Random => Vector3::new(state.rng.gen(), state.rng.gen(), state.rng.gen()),
    }
  }
}
