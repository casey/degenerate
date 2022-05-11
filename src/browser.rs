use {
  self::{cast::Cast, get_document::GetDocument, select::Select, stderr::Stderr, window::window},
  super::*,
  std::sync::atomic::{AtomicBool, Ordering},
  wasm_bindgen::{closure::Closure, JsCast},
  web_sys::{
    CanvasRenderingContext2d, Document, Element, HtmlCanvasElement, HtmlTextAreaElement, Node,
    Window,
  },
};

pub(crate) use display::Display;

mod cast;
mod display;
mod get_document;
mod select;
mod stderr;
mod window;

static RESIZE: AtomicBool = AtomicBool::new(true);

macro_rules! log {
  ($($t:tt)*) => (web_sys::console::log_1(&format_args!($($t)*).to_string().into()))
}

pub(crate) fn run() {
  if let Err(err) = run_inner() {
    Stderr::get().unwrap().set(err.as_ref());
  }
}

fn run_inner() -> Result {
  console_error_panic_hook::set_once();

  let window = window();

  let document = window.get_document();

  let textarea = document.select("textarea")?.cast::<HtmlTextAreaElement>()?;

  let canvas = document.select("canvas")?.cast::<HtmlCanvasElement>()?;

  let stderr = Stderr::get()?;

  let device_pixel_ratio = window.device_pixel_ratio();

  let context = canvas
    .get_context("2d")
    .map_err(|err| format!("`canvas.get_context(\"2d\")` failed: {:?}", err))?
    .ok_or_else(|| format!("failed to retrieve context"))?
    .cast::<CanvasRenderingContext2d>()?;

  let display = Display { context };

  let nav = document.select("nav")?.cast::<Element>()?;

  let textarea_clone = textarea.clone();
  let cb = Closure::wrap(Box::new(move || {
    if RESIZE.load(Ordering::Relaxed) {
      let css_pixel_height: f64 = canvas.client_height().try_into().unwrap();
      let css_pixel_width: f64 = canvas.client_width().try_into().unwrap();
      let device_pixel_height = css_pixel_height * device_pixel_ratio;
      let device_pixel_width = css_pixel_width * device_pixel_ratio;
      let height = if cfg!(debug_assertions) {
        device_pixel_height / 32.0
      } else {
        device_pixel_height
      };
      let width = if cfg!(debug_assertions) {
        device_pixel_width / 32.0
      } else {
        device_pixel_width
      };
      canvas.set_height(height.ceil() as u32);
      canvas.set_width(width.ceil() as u32);
      RESIZE.store(false, Ordering::Relaxed);
    }
    nav.set_class_name("fade-out");
    match Computer::run(&display, textarea_clone.value().split_whitespace()) {
      Err(err) => stderr.set(err.as_ref()),
      Ok(()) => stderr.clear(),
    }
  }) as Box<dyn FnMut()>);
  textarea
    .add_event_listener_with_callback("input", &cb.as_ref().dyn_ref().unwrap())
    .map_err(|err| format!("failed to set textarea listener: {:?}", err))?;
  cb.forget();

  let cb = Closure::wrap(Box::new(move || {
    log!("resize");
    RESIZE.store(true, Ordering::Relaxed)
  }) as Box<dyn FnMut()>);
  window
    .add_event_listener_with_callback("resize", &cb.as_ref().dyn_ref().unwrap())
    .map_err(|err| format!("failed to set textarea listener: {:?}", err))?;
  cb.forget();

  Ok(())
}
