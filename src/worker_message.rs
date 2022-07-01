use super::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum WorkerMessage {
  Checkbox(String),
  Clear,
  Done,
  Render(State),
  Resolution(u32),
  Save,
}
