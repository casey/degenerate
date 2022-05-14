use {
  self::{
    add_event_listener::AddEventListener, app::App, cast::Cast, get_document::GetDocument,
    js_value_error::JsValueError, select::Select, stderr::Stderr, window::window,
  },
  super::*,
  std::{
    fmt::{self, Formatter},
    ops::Deref,
    sync::{Arc, Mutex},
  },
  wasm_bindgen::{closure::Closure, JsCast, JsValue},
  web_sys::{
    CanvasRenderingContext2d, Document, Element, EventTarget, HtmlCanvasElement, HtmlElement,
    HtmlTextAreaElement, Window,
  },
};

pub(crate) use display::Display;

mod add_event_listener;
mod app;
mod cast;
mod display;
mod get_document;
mod js_value_error;
mod select;
mod stderr;
mod window;

pub(crate) fn run() {
  console_error_panic_hook::set_once();
  console_log::init_with_level(log::Level::Trace).unwrap();
  Stderr::get().update(App::init());
}
