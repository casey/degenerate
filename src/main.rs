use {
  crate::{
    add_event_listener::AddEventListener, app::App, app_message::AppMessage, cast::Cast,
    get_document::GetDocument, gpu::Gpu, js_value_error::JsValueError, select::Select,
    state::State, stderr::Stderr, window::window, worker_message::WorkerMessage, add_class::AddClass
  },
  nalgebra::{Rotation3, Similarity2, UnitComplex, Vector3},
  serde::{Deserialize, Serialize},
  std::{
    cell::Cell,
    collections::BTreeMap,
    f32,
    fmt::{self, Formatter},
    ops::Deref,
    string::ToString,
    sync::{Arc, Mutex},
  },
  wasm_bindgen::{closure::Closure, JsCast, JsValue},
  web_sys::{
    Document, Element, EventTarget, HtmlCanvasElement, HtmlElement, HtmlTextAreaElement,
    MessageEvent, WebGl2RenderingContext, WebGlContextAttributes, WebGlFramebuffer, WebGlTexture,
    WebGlUniformLocation, Window, Worker,
  },
};

type Error = Box<dyn std::error::Error>;
type Result<T = (), E = Error> = std::result::Result<T, E>;

mod add_event_listener;
mod add_class;
mod app;
mod app_message;
mod cast;
mod get_document;
mod gpu;
mod js_value_error;
mod select;
mod state;
mod stderr;
mod window;
mod worker_message;

fn main() {
  console_error_panic_hook::set_once();
  console_log::init_with_level(log::Level::Trace).unwrap();
  Stderr::get().update(App::init());
}
