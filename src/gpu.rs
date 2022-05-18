use super::*;

pub(crate) trait Gpu {
  fn apply(&self, state: &Computer) -> Result;
}
