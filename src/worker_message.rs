use super::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum WorkerMessage {
  Checkbox(String),
  Clear,
  Done,
  Error(String),
  OscillatorFrequency(f32),
  OscillatorGain(f32),
  Radio(String, Vec<String>),
  Record(bool),
  Render(State),
  Resolution(u32),
  Save,
}
