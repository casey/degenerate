use super::*;

#[derive(Copy, Clone, Debug)]
pub(crate) enum Operation {
  Random,
  Invert,
}

impl Operation {
  pub(crate) fn apply(self, rng: &mut impl Rng, element: Vector3<u8>) -> Vector3<u8> {
    match self {
      Self::Invert => element.map(|scalar| !scalar),
      Self::Random => Vector3::new(rng.gen(), rng.gen(), rng.gen()),
    }
  }
}
