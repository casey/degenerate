use super::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum WorkerMessage {
  Checkbox(String),
  Clear,
  DecibelRange {
    min: f32,
    max: f32,
  },
  Done,
  Error(String),
  OscillatorFrequency(f32),
  OscillatorGain(f32),
  Radio(String, Vec<String>),
  Record,
  Render(Filter),
  Resolution(u32),
  Save,
  Slider {
    initial: f64,
    max: f64,
    min: f64,
    name: String,
    step: f64,
  },
}
