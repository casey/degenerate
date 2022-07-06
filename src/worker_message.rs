use super::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum WorkerMessage {
  Checkbox(String),
  Error(String),
  Clear,
  Done,
  Error(String),
  Radio(String, Vec<String>),
  Render(State),
  Resolution(u32),
  Save,
}
