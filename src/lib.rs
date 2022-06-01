use {
  crate::{
    add_event_listener::AddEventListener,
    app::App,
    cast::Cast,
    get_document::GetDocument,
    gpu::Gpu,
    js_value_error::JsValueError,
    message::{Message, MessageType},
    select::Select,
    state::State,
    stderr::Stderr,
    window::window,
  },
  nalgebra::{Similarity2, UnitComplex},
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
  wasm_bindgen::{closure::Closure, prelude::wasm_bindgen, JsCast, JsValue},
  web_sys::{
    Document, Element, EventTarget, HtmlCanvasElement, HtmlElement, HtmlTextAreaElement,
    WebGl2RenderingContext, WebGlContextAttributes, WebGlFramebuffer, WebGlTexture,
    WebGlUniformLocation, Window, Worker,
  },
};

type Error = Box<dyn std::error::Error>;
type Result<T = (), E = Error> = std::result::Result<T, E>;

mod add_event_listener;
mod app;
mod cast;
mod get_document;
pub mod gpu;
mod js_value_error;
mod message;
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
