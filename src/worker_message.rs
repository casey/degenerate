use super::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum WorkerMessage {
  Clear,
  Done,
  OscillatorFrequency(f32),
  Record,
  Render(State),
  Resolution(u32),
  Save,
}
