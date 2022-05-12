use {
  self::{
    add_event_listener::AddEventListener, app::App, cast::Cast, get_document::GetDocument,
    select::Select, stderr::Stderr, window::window,
  },
  super::*,
  std::{
    ops::Deref,
    sync::{Arc, Mutex},
  },
  wasm_bindgen::{closure::Closure, JsCast},
  web_sys::{
    CanvasRenderingContext2d, Document, Element, EventTarget, HtmlCanvasElement, HtmlElement,
    HtmlTextAreaElement, Window,
  },
};

pub(crate) use display::Display;

macro_rules! log {
  ($($t:tt)*) => (web_sys::console::log_1(&format_args!($($t)*).to_string().into()))
}

mod add_event_listener;
mod app;
mod cast;
mod display;
mod get_document;
mod select;
mod stderr;
mod window;

pub(crate) fn run() {
  console_error_panic_hook::set_once();

  if let Err(err) = App::init() {
    Stderr::get().unwrap().set(err.as_ref());
  }
}
