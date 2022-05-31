use super::*;

#[derive(Debug, Deserialize)]
pub(crate) struct State {
  pub(crate) alpha: f64,
  pub(crate) mask: Mask,
  pub(crate) operation: Operation,
  pub(crate) wrap: bool
}
