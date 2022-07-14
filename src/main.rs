use {
  crate::{
    add_event_listener::AddEventListener, app::App, app_message::AppMessage, cast::Cast,
    error::Error, filter::Filter, get_document::GetDocument, gpu::Gpu, select::Select,
    stderr::Stderr, window::window, worker_message::WorkerMessage,
  },
  image::{ImageBuffer, ImageError, ImageOutputFormat},
  js_sys::{Float32Array, Promise},
  lazy_static::lazy_static,
  nalgebra::{Similarity2, UnitComplex},
  serde::{Deserialize, Serialize},
  std::{
    collections::BTreeMap,
    convert::Infallible,
    f32,
    fmt::{self, Display, Formatter},
    io::Cursor,
    mem,
    num::TryFromIntError,
    ops::Deref,
    str,
    string::ToString,
    sync::{Arc, Mutex},
  },
  wasm_bindgen::{closure::Closure, convert::FromWasmAbi, JsCast, JsValue},
  web_sys::{
    AnalyserNode, AudioContext, Document, EventTarget, GainNode, HtmlAnchorElement,
    HtmlButtonElement, HtmlCanvasElement, HtmlDivElement, HtmlElement, HtmlInputElement,
    HtmlLabelElement, HtmlOptionElement, HtmlSelectElement, HtmlSpanElement, HtmlTextAreaElement,
    KeyboardEvent, MediaStream, MediaStreamConstraints, MessageEvent, OscillatorNode,
    WebGl2RenderingContext, WebGlContextAttributes, WebGlFramebuffer, WebGlTexture,
    WebGlUniformLocation, Window, Worker,
  },
  widget::Widget,
};

type Result<T = (), E = Error> = std::result::Result<T, E>;

mod add_event_listener;
mod app;
mod app_message;
mod cast;
mod error;
mod filter;
mod get_document;
mod gpu;
mod select;
mod stderr;
mod widget;
mod window;
mod worker_message;

fn main() {
  console_error_panic_hook::set_once();
  console_log::init_with_level(log::Level::Trace).unwrap();
  Stderr::get().update(App::init());
}
