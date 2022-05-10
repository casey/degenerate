use {
  self::{cast::Cast, get_document::GetDocument, select::Select, stderr::Stderr, window::window},
  super::*,
  wasm_bindgen::{closure::Closure, JsCast},
  web_sys::{
    CanvasRenderingContext2d, Document, Element, HtmlCanvasElement, HtmlTextAreaElement, Node,
    Window,
  },
};

pub(crate) use display::Display;

// todo:
// - deploy to pages
// - remove unwraps
// - render in background thread?
// - copy static assets
// - deploy with manual

mod cast;
mod display;
mod get_document;
mod select;
mod stderr;
mod window;

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

  let css_pixel_height: f64 = canvas.client_height().try_into()?;
  let css_pixel_width: f64 = canvas.client_width().try_into()?;
  let device_pixel_ratio = window.device_pixel_ratio();
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

  let context = canvas
    .get_context("2d")
    .map_err(|err| format!("`canvas.get_context(\"2d\")` failed: {:?}", err))?
    .ok_or_else(|| format!("failed to retrieve context"))?
    .cast::<CanvasRenderingContext2d>()?;

  let display = Display { context };

  let textarea_clone = textarea.clone();
  let cb = Closure::wrap(Box::new(move || {
    document
      .select("main")
      .unwrap()
      .cast::<Element>()
      .unwrap()
      .set_class_name("fade-out");

    match Computer::run(&display, textarea_clone.value().split_whitespace()) {
      Err(err) => stderr.set(err.as_ref()),
      Ok(()) => stderr.clear(),
    }
  }) as Box<dyn FnMut()>);

  textarea
    .add_event_listener_with_callback("input", &cb.as_ref().dyn_ref().unwrap())
    .map_err(|err| format!("failed to set textarea listener: {:?}", err))?;

  cb.forget();

  Ok(())
}
