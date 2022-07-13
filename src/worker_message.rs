use super::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum WorkerMessage {
  Clear,
  DecibelRange { min: f32, max: f32 },
  Done,
  Error(String),
  OscillatorFrequency(f32),
  OscillatorGain(f32),
  Record,
  Render(Filter),
  Resolution(u32),
  Save,
  Widget { name: String, widget: Widget },
}
