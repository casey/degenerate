use super::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum WorkerMessage {
  Checkbox(String),
  Clear,
  Done,
  OscillatorFrequency(f32),
  Record(bool),
  Render(State),
  Resolution(u32),
  Save,
}
