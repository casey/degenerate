use super::*;

pub(crate) struct Software;

impl Gpu for Software {
  fn apply(&self, _state: &Computer) -> Result {
    Ok(())
  }
}

impl Software {
  pub(crate) fn new() -> Self {
    Self
  }
}
