use {
  serde::{Deserialize, Serialize},
  std::cell::RefCell,
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

thread_local! {
  static SYSTEM: RefCell<System> = RefCell::new(System::new());
}

pub fn send(message: Message) {
  SYSTEM.with(|cell| cell.borrow_mut().send(message));
}

pub fn error(message: impl ToString) {
  SYSTEM.with(|cell| cell.borrow_mut().send(Message::Error(message.to_string())));
}

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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
  pub alpha: f32,
  pub color_transform: Matrix4,
  pub position_transform: Matrix3,
  pub coordinates: bool,
  pub default_color: [f32; 3],
  pub field: Field,
  pub times: u32,
  pub wrap: bool,
}

impl Filter {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn x(self) -> Self {
    Self {
      field: Field::X,
      ..self
    }
  }

  pub fn circle(self) -> Self {
    Self {
      field: Field::Circle,
      ..self
    }
  }

  pub fn position(self, position_transform: impl Into<Matrix3>) -> Self {
    Self {
      position_transform: position_transform.into(),
      ..self
    }
  }

  pub fn color(self, color_transform: impl Into<Matrix4>) -> Self {
    Self {
      color_transform: color_transform.into(),
      ..self
    }
  }

  pub fn alpha(self, alpha: f32) -> Self {
    Self { alpha, ..self }
  }

  pub fn wrap(self, wrap: bool) -> Self {
    Self { wrap, ..self }
  }

  pub fn times(self, times: u32) -> Self {
    Self { times, ..self }
  }

  pub fn render(self) -> Self {
    SYSTEM.with(|cell| cell.borrow_mut().send(Message::Render(self.clone())));
    self
  }
}

impl Default for Filter {
  fn default() -> Self {
    Self {
      alpha: 1.0,
      color_transform: Similarity3::from_scaling(-1.0).into(),
      position_transform: Matrix3::identity(),
      coordinates: false,
      default_color: [0.0, 0.0, 0.0],
      field: Field::All,
      times: 1,
      wrap: false,
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Default, Copy, Clone)]
pub struct Frame {
  pub delta: f32,
  pub number: u64,
  pub time: f32,
}

pub struct System {
  scope: DedicatedWorkerGlobalScope,
  listener: Option<Closure<dyn FnMut(MessageEvent)>>,
}

impl System {
  fn new() -> Self {
    Self {
      scope: js_sys::global().dyn_into().unwrap(),
      listener: None,
    }
  }

  fn execute_inner(&mut self, process: Box<dyn Process + 'static>) {
    let mut frame = Frame::default();

    if let Some(listener) = &self.listener {
      self
        .scope
        .remove_event_listener_with_callback("message", listener.as_ref().dyn_ref().unwrap())
        .unwrap()
    }

    self.listener = None;

    let closure = Closure::wrap(Box::new(move |e: MessageEvent| {
      let event = serde_json::from_str(&e.data().as_string().unwrap()).unwrap();

      if let Event::Frame(time) = event {
        frame.delta = time - frame.time;
        frame.time = time;
        if process.clear() {
          SYSTEM.with(|system| system.borrow_mut().send(Message::Clear));
        }
        process.frame(frame);
        frame.number += 1;
      }
    }) as Box<dyn FnMut(MessageEvent)>);

    self
      .scope
      .add_event_listener_with_callback("message", closure.as_ref().dyn_ref().unwrap())
      .unwrap();

    self.listener = Some(closure);
  }

  fn send(&self, message: Message) {
    self
      .scope
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

pub trait Process {
  fn clear(&self) -> bool {
    true
  }

  fn frame(&self, frame: Frame);

  fn execute(self)
  where
    Self: Sized + 'static,
  {
    SYSTEM.with(|cell| cell.borrow_mut().execute_inner(Box::new(self)));
  }
}

impl<T> Process for T
where
  T: Fn(Frame),
{
  fn frame(&self, frame: Frame) {
    self(frame)
  }
}
