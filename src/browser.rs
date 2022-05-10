use {
  super::*,
  wasm_bindgen::{closure::Closure, JsCast},
  web_sys::{CanvasRenderingContext2d, Document, Element, HtmlCanvasElement, HtmlTextAreaElement},
};

// todo:
// - remove unwraps
// - surface errors to user
// - decide canvas layout

pub(crate) mod display;

pub(crate) fn run() {
  run_inner().unwrap();
}

trait Select {
  fn select(&self, selector: &str) -> Result<Element>;
}

impl Select for Document {
  fn select(&self, selector: &str) -> Result<Element> {
    Ok(self.query_selector(selector).unwrap().unwrap())
  }
}

trait Cast {
  fn cast<T: JsCast>(self) -> Result<T>;
}

impl<V: JsCast + std::fmt::Debug> Cast for V {
  fn cast<T: JsCast>(self) -> Result<T> {
    Ok(self.dyn_into::<T>().unwrap())
  }
}

fn run_inner() -> Result<(), Error> {
  console_error_panic_hook::set_once();

  let window = web_sys::window().ok_or_else(|| "window missing".to_string())?;

  let document = window
    .document()
    .ok_or_else(|| "window.document missing".to_string())?;

  let textarea = document.select("textarea")?.cast::<HtmlTextAreaElement>()?;

  let canvas = document.select("canvas")?.cast::<HtmlCanvasElement>()?;

  let context = canvas
    .get_context("2d")
    .unwrap()
    .unwrap()
    .cast::<CanvasRenderingContext2d>()?;

  let display = Display { context };

  let textarea_clone = textarea.clone();
  let cb = Closure::wrap(Box::new(move || {
    Computer::run(&display, textarea_clone.value().split_whitespace()).unwrap();
  }) as Box<dyn FnMut()>);

  textarea
    .add_event_listener_with_callback("input", &cb.as_ref().unchecked_ref())
    .unwrap();

  cb.forget();

  web_sys::console::log_1(&"hello".into());

  Ok(())
}
