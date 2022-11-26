use {
  serde::{Deserialize, Serialize},
  wasm_bindgen::{closure::Closure, JsCast, JsValue},
  web_sys::{DedicatedWorkerGlobalScope, MessageEvent},
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "tag", content = "content")]
pub enum AppMessage {
  Frame,
  Program(String),
  Widget {
    key: String,
    value: serde_json::Value,
  },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum WorkerMessage {
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Widget {
  Checkbox,
  Slider {
    initial: f64,
    max: f64,
    min: f64,
    step: f64,
  },
  Radio {
    options: Vec<String>,
  },
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
  pub alpha: f32,
  pub color_transform: [f32; 16],
  pub coordinate_transform: [f32; 9],
  pub coordinates: bool,
  pub default_color: [f32; 3],
  pub field: u32,
  pub field_mod_divisor: u32,
  pub field_mod_remainder: u32,
  pub field_rows_rows: u32,
  pub field_rows_step: u32,
  pub wrap: bool,
}

pub trait Program {
  fn new() -> Self
  where
    Self: Sized;

  fn on_message(&mut self, message: AppMessage);
}

pub fn execute<T: Program + 'static>() {
  execute_inner(Box::new(T::new()));
}

pub fn post_message(worker_message: WorkerMessage) {
  let global = js_sys::global().unchecked_into::<DedicatedWorkerGlobalScope>();

  global
    .post_message(&JsValue::from_str(
      &serde_json::to_string(&worker_message).unwrap(),
    ))
    .unwrap();
}

fn execute_inner(mut program: Box<dyn Program>) {
  let closure = Closure::wrap(Box::new(move |e: MessageEvent| {
    program.on_message(serde_json::from_str(&e.data().as_string().unwrap()).unwrap())
  }) as Box<dyn FnMut(MessageEvent)>);

  js_sys::global()
    .unchecked_into::<DedicatedWorkerGlobalScope>()
    .add_event_listener_with_callback("message", closure.as_ref().dyn_ref().unwrap())
    .unwrap();

  closure.forget();
}
