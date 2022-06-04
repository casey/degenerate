use super::*;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct State {
  pub(crate) alpha: f32,
  pub(crate) default_color: [f32; 3],
  pub(crate) mask: u32,
  pub(crate) mask_mod_divisor: u32,
  pub(crate) mask_mod_remainder: u32,
  pub(crate) mask_rows_rows: u32,
  pub(crate) mask_rows_step: u32,
  pub(crate) operation: u32,
  pub(crate) operation_rotate_color_axis: String,
  pub(crate) operation_rotate_color_turns: f32,
  pub(crate) rotation: f32,
  pub(crate) scale: f32,
  pub(crate) wrap: bool,
}
