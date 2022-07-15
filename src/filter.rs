use super::*;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Filter {
  pub(crate) alpha: f32,
  pub(crate) color_transform: [f32; 16],
  pub(crate) coordinates: bool,
  pub(crate) default_color: [f32; 3],
  pub(crate) rotation: f32,
  pub(crate) scale: f32,
  pub(crate) field: u32,
  pub(crate) field_mod_divisor: u32,
  pub(crate) field_mod_remainder: u32,
  pub(crate) field_rows_rows: u32,
  pub(crate) field_rows_step: u32,
  pub(crate) wrap: bool,
}
