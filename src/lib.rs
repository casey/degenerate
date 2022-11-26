use {
  serde::{Deserialize, Serialize},
  wasm_bindgen::{closure::Closure, JsCast, JsValue},
  web_sys::{DedicatedWorkerGlobalScope, MessageEvent},
};

#[derive(Serialize, Deserialize)]
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

pub struct System {
  dedicated_worker_global_scope: DedicatedWorkerGlobalScope,
}

impl System {
  fn new() -> Self {
    Self {
      dedicated_worker_global_scope: js_sys::global()
        .unchecked_into::<DedicatedWorkerGlobalScope>(),
    }
  }

  fn execute(mut program: Box<dyn Process>) {
    let closure = Closure::wrap(Box::new(move |e: MessageEvent| {
      program.on_message(serde_json::from_str(&e.data().as_string().unwrap()).unwrap())
    }) as Box<dyn FnMut(MessageEvent)>);

    js_sys::global()
      .unchecked_into::<DedicatedWorkerGlobalScope>()
      .add_event_listener_with_callback("message", closure.as_ref().dyn_ref().unwrap())
      .unwrap();

    closure.forget();
  }

  fn post_message(&self, worker_message: WorkerMessage) {
    self
      .dedicated_worker_global_scope
      .post_message(&JsValue::from_str(
        &serde_json::to_string(&worker_message).unwrap(),
      ))
      .unwrap();
  }

  pub fn render(&self, filter: Filter) {
    self.post_message(WorkerMessage::Render(filter));
  }

  pub fn save(&self) {
    self.post_message(WorkerMessage::Save);
  }
}

#[derive(Debug, Serialize, Deserialize)]
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

impl Default for Filter {
  fn default() -> Self {
    Self {
      alpha: 1.0,
      color_transform: [
        -1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
      ],
      coordinate_transform: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
      coordinates: false,
      default_color: [0.0, 0.0, 0.0],
      field: 0,
      field_mod_divisor: 0,
      field_mod_remainder: 0,
      field_rows_rows: 0,
      field_rows_step: 0,
      wrap: false,
    }
  }
}

pub trait Process {
  fn new(system: System) -> Self
  where
    Self: Sized;

  fn execute()
  where
    Self: Sized + 'static,
  {
    System::execute(Box::new(Self::new(System::new())));
  }

  fn on_frame(&mut self) {}

  fn on_message(&mut self, message: AppMessage) {
    if let AppMessage::Frame = message {
      self.on_frame();
    }
  }
}

#[derive(Serialize, Deserialize)]
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

impl Widget {
  pub fn id(&self, name: &str) -> String {
    format!("widget-{}-{name}", self.kind())
  }

  pub fn key(&self, name: &str) -> String {
    format!("{}-{name}", self.kind())
  }

  fn kind(&self) -> &str {
    match self {
      Self::Checkbox => "checkbox",
      Self::Slider { .. } => "slider",
      Self::Radio { .. } => "radio",
    }
  }
}

#[derive(Serialize, Deserialize)]
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
