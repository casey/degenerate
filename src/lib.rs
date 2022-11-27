use {
  serde::{Deserialize, Serialize},
  wasm_bindgen::{closure::Closure, JsCast, JsValue},
  web_sys::{DedicatedWorkerGlobalScope, MessageEvent},
};

pub use std::f32::consts::TAU;

pub type Matrix3 = nalgebra::Matrix3<f32>;
pub type Matrix4 = nalgebra::Matrix4<f32>;
pub type Rotation2 = nalgebra::Rotation2<f32>;
pub type Rotation3 = nalgebra::Rotation3<f32>;
pub type Scale2 = nalgebra::Scale2<f32>;
pub type Similarity2 = nalgebra::Similarity2<f32>;
pub type Similarity3 = nalgebra::Similarity3<f32>;
pub type Translation2 = nalgebra::Translation2<f32>;
pub type Vector3 = nalgebra::Vector3<f32>;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "tag", content = "content")]
pub enum Event {
  Frame(f32),
  Script(String),
  Widget {
    key: String,
    value: serde_json::Value,
  },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
  pub alpha: f32,
  pub color_transform: Matrix4,
  pub coordinate_transform: Matrix3,
  pub coordinates: bool,
  pub default_color: [f32; 3],
  pub field: Field,
  pub wrap: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Field {
  All,
  Check,
  Circle,
  Cross,
  Equalizer,
  Frequency,
  Mod { divisor: u32, remainder: u32 },
  Rows { on: u32, off: u32 },
  Square,
  TimeDomain,
  Top,
  Wave,
  X,
}

impl Default for Filter {
  fn default() -> Self {
    Self {
      alpha: 1.0,
      color_transform: Similarity3::from_scaling(-1.0).into(),
      coordinate_transform: Matrix3::identity(),
      coordinates: false,
      default_color: [0.0, 0.0, 0.0],
      field: Field::All,
      wrap: false,
    }
  }
}

pub struct System {
  dedicated_worker_global_scope: DedicatedWorkerGlobalScope,
  frame: u64,
  delta: f32,
  last_frame: f32,
}

impl System {
  fn new() -> Self {
    Self {
      dedicated_worker_global_scope: js_sys::global()
        .unchecked_into::<DedicatedWorkerGlobalScope>(),
      frame: 0,
      delta: 0.0,
      last_frame: 0.0,
    }
  }

  pub fn execute<T: Fn(&System, &Event) + 'static>(f: T) {
    let mut system = System::new();

    let closure = Closure::wrap(Box::new(move |e: MessageEvent| {
      let event = serde_json::from_str(&e.data().as_string().unwrap()).unwrap();

      if let Event::Frame(timestamp) = event {
        system.delta = timestamp - system.last_frame;
      }

      f(&system, &event);

      if let Event::Frame(timestamp) = event {
        system.frame += 1;
        system.last_frame = timestamp;
      }
    }) as Box<dyn FnMut(MessageEvent)>);

    js_sys::global()
      .unchecked_into::<DedicatedWorkerGlobalScope>()
      .add_event_listener_with_callback("message", closure.as_ref().dyn_ref().unwrap())
      .unwrap();

    closure.forget();
  }

  pub fn frame(&self) -> u64 {
    self.frame
  }

  pub fn delta(&self) -> f32 {
    self.delta
  }

  pub fn send(&self, message: Message) {
    self
      .dedicated_worker_global_scope
      .post_message(&JsValue::from_str(
        &serde_json::to_string(&message).unwrap(),
      ))
      .unwrap();
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
pub enum Message {
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
