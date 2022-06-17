use super::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum WorkerMessage {
  Clear,
  Done,
  Render(State),
  Resolution(u32),
  Save,
}
