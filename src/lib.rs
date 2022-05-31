use {
  crate::{
    add_event_listener::AddEventListener, app::App, cast::Cast, color_axis::ColorAxis,
    command::Command, computer::Computer, get_document::GetDocument, gpu::Gpu,
    js_value_error::JsValueError, mask::Mask, operation::Operation, select::Select, state::State,
    stderr::Stderr, window::window,
  },
  nalgebra::{Rotation3, Similarity2, UnitComplex, Vector3, Vector4},
  rand::{rngs::StdRng, seq::SliceRandom, SeedableRng},
  serde::{Deserialize, Serialize},
  std::{
    cell::Cell,
    collections::BTreeMap,
    f32,
    fmt::{self, Formatter},
    ops::Deref,
    str::FromStr,
    string::ToString,
    sync::{Arc, Mutex},
  },
  strum::{AsRefStr, EnumString, EnumVariantNames, VariantNames},
  wasm_bindgen::{closure::Closure, prelude::wasm_bindgen, JsCast, JsValue},
  web_sys::{
    Document, Element, EventTarget, HtmlCanvasElement, HtmlElement, HtmlTextAreaElement,
    WebGl2RenderingContext, WebGlContextAttributes, WebGlFramebuffer, WebGlProgram, WebGlTexture,
    WebGlUniformLocation, Window, Worker,
  },
};

type Error = Box<dyn std::error::Error>;
type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

mod add_event_listener;
mod app;
mod cast;
mod color_axis;
mod command;
mod computer;
mod get_document;
pub mod gpu;
mod js_value_error;
mod mask;
mod operation;
mod select;
mod state;
mod stderr;
pub mod test;
mod window;

#[wasm_bindgen]
pub fn run() {
  console_error_panic_hook::set_once();
  console_log::init_with_level(log::Level::Trace).unwrap();
  Stderr::get().update(App::init());
}
