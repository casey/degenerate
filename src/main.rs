use {
  crate::{
    add_event_listener::AddEventListener, app::App, app_message::AppMessage, cast::Cast,
    filter::Filter, get_document::GetDocument, gpu::Gpu, js_value_error::JsValueError,
    select::Select, stderr::Stderr, window::window, worker_message::WorkerMessage,
  },
  image::{ImageBuffer, ImageOutputFormat},
  js_sys::{Float32Array, Promise},
  lazy_static::lazy_static,
  nalgebra::{Similarity2, UnitComplex},
  serde::{Deserialize, Serialize},
  std::{
    collections::BTreeMap,
    f32,
    fmt::{self, Formatter},
    io::Cursor,
    mem,
    ops::Deref,
    string::ToString,
    sync::{Arc, Mutex},
  },
  wasm_bindgen::{closure::Closure, convert::FromWasmAbi, JsCast, JsValue},
  web_sys::{
    AnalyserNode, AudioContext, Document, Element, EventTarget, GainNode, HtmlAnchorElement,
    HtmlButtonElement, HtmlCanvasElement, HtmlDivElement, HtmlElement, HtmlInputElement,
    HtmlLabelElement, HtmlOptionElement, HtmlSelectElement, HtmlSpanElement, HtmlTextAreaElement,
    KeyboardEvent, MediaStream, MediaStreamConstraints, MessageEvent, OscillatorNode,
    WebGl2RenderingContext, WebGlContextAttributes, WebGlFramebuffer, WebGlTexture,
    WebGlUniformLocation, Window, Worker,
  },
};

type Error = Box<dyn std::error::Error>;
type Result<T = (), E = Error> = std::result::Result<T, E>;

mod add_event_listener;
mod app;
mod app_message;
mod cast;
mod filter;
mod get_document;
mod gpu;
mod js_value_error;
mod select;
mod stderr;
mod window;
mod worker_message;

fn main() {
  console_error_panic_hook::set_once();
  console_log::init_with_level(log::Level::Trace).unwrap();
  Stderr::get().update(App::init());
}
