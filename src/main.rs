use {
  crate::{
    add_event_listener::AddEventListener, app::App, app_message::AppMessage, cast::Cast,
    get_document::GetDocument, gpu::Gpu, js_value_error::JsValueError, select::Select,
    state::State, stderr::Stderr, window::window, worker_message::WorkerMessage,
  },
  image::{ImageBuffer, ImageOutputFormat},
  include_dir::include_dir,
  js_sys::{Float32Array, Promise},
  nalgebra::{Rotation3, Similarity2, UnitComplex, Vector3},
  serde::{Deserialize, Serialize},
  std::{
    collections::BTreeMap,
    f32,
    fmt::{self, Formatter},
    io::Cursor,
    mem,
    ops::Deref,
    path::Path,
    string::ToString,
    sync::{Arc, Mutex},
  },
  wasm_bindgen::{closure::Closure, convert::FromWasmAbi, JsCast, JsValue},
  web_sys::{
    AnalyserNode, AudioContext, Document, Document, Document, Element, Element, Element,
    EventTarget, EventTarget, EventTarget, HtmlAnchorElement, HtmlAnchorElement, HtmlAnchorElement,
    HtmlButtonElement, HtmlCanvasElement, HtmlCanvasElement, HtmlCanvasElement, HtmlDivElement,
    HtmlDivElement, HtmlDivElement, HtmlElement, HtmlElement, HtmlElement, HtmlInputElement,
    HtmlInputElement, HtmlInputElement, HtmlLabelElement, HtmlLabelElement, HtmlLabelElement,
    HtmlOptionElement, HtmlOptionElement, HtmlOptionElement, HtmlSelectElement, HtmlSelectElement,
    HtmlSelectElement, HtmlTextAreaElement, HtmlTextAreaElement, HtmlTextAreaElement,
    KeyboardEvent, KeyboardEvent, KeyboardEvent, MediaStream, MediaStreamAudioSourceNode,
    MediaStreamConstraints, MediaStreamTrack, MessageEvent, MessageEvent, MessageEvent,
    OscillatorNode, WebGl2RenderingContext, WebGl2RenderingContext, WebGl2RenderingContext,
    WebGlContextAttributes, WebGlContextAttributes, WebGlContextAttributes, WebGlFramebuffer,
    WebGlFramebuffer, WebGlFramebuffer, WebGlTexture, WebGlTexture, WebGlTexture,
    WebGlUniformLocation, WebGlUniformLocation, WebGlUniformLocation, Window, Window, Window,
    Worker, Worker, Worker,
  },
};

type Error = Box<dyn std::error::Error>;
type Result<T = (), E = Error> = std::result::Result<T, E>;

mod add_event_listener;
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
