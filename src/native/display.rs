use super::*;

pub(crate) struct Display;

impl Display {
  pub(crate) fn render(&self, _memory: &DMatrix<Vector3<u8>>) {}

  pub(crate) fn dimensions(&self) -> (usize, usize) {
    (256, 256)
  }
}
