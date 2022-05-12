use super::*;

pub(crate) struct Display;

impl Display {
  pub(crate) fn render(&self, _memory: &DMatrix<Vector3<u8>>) -> Result {
    Ok(())
  }

  pub(crate) fn dimensions(&self) -> Result<(usize, usize)> {
    Ok((256, 256))
  }
}
