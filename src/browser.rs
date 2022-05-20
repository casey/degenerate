pub(crate) use {
  self::{
    add_event_listener::AddEventListener, app::App, cast::Cast, get_document::GetDocument,
    gpu::Gpu, js_value_error::JsValueError, select::Select, stderr::Stderr, window::window,
  },
  super::*,
  js_sys::Float32Array,
  std::{
    cell::Cell,
    ops::Deref,
    sync::{Arc, Mutex},
  },
  wasm_bindgen::{closure::Closure, prelude::wasm_bindgen, JsCast, JsValue},
  web_sys::{
    CanvasRenderingContext2d, Document, Element, EventTarget, HtmlCanvasElement, HtmlElement,
    HtmlTextAreaElement, ImageData, WebGl2RenderingContext, WebGlFramebuffer, WebGlProgram,
    WebGlTexture, Window,
  },
};

mod add_event_listener;
mod app;
mod cast;
mod get_document;
pub mod gpu;
mod js_value_error;
mod select;
mod stderr;
pub mod test;
mod window;

#[wasm_bindgen]
pub fn run() {
  console_error_panic_hook::set_once();
  console_log::init_with_level(log::Level::Trace).unwrap();
  Stderr::get().update(App::init());
}
