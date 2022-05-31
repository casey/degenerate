use super::*;

#[derive(Debug, Deserialize)]
pub(crate) struct State {
  pub(crate) alpha: f64,
  pub(crate) default_color: [f32; 3],
  pub(crate) mask: Mask,
  pub(crate) operation: Operation,
  pub(crate) rotation: f32,
  pub(crate) scale: f32,
  pub(crate) wrap: bool,
}
