use {
  super::*,
  wasm_bindgen::{closure::Closure, JsCast},
  web_sys::{CanvasRenderingContext2d, Document, Element, HtmlCanvasElement, HtmlTextAreaElement},
};

// todo:
// - decide canvas layout
// - surface errors to user
// - deploy to pages

pub(crate) mod display;

pub(crate) fn run() {
  run_inner().unwrap();
}

trait Select {
  fn select(&self, selector: &str) -> Result<Element>;
}

impl Select for Document {
  fn select(&self, selector: &str) -> Result<Element> {
    Ok(
      self
        .query_selector(selector)
        .map_err(|err| format!("`select` failed: {:?}", err))?
        .ok_or_else(|| format!("selector `{}` returned no elements", selector))?,
    )
  }
}

trait Cast {
  fn cast<T: JsCast>(self) -> Result<T>;
}

impl<V: JsCast + std::fmt::Debug> Cast for V {
  fn cast<T: JsCast>(self) -> Result<T> {
    Ok(
      self
        .dyn_into::<T>()
        .map_err(|err| format!("`cast` failed: {:?}", err))?,
    )
  }
}

fn run_inner() -> Result {
  console_error_panic_hook::set_once();

  let window = web_sys::window().ok_or_else(|| "`window` missing".to_string())?;

  let document = window
    .document()
    .ok_or_else(|| "`window.document` missing".to_string())?;

  let textarea = document.select("textarea")?.cast::<HtmlTextAreaElement>()?;

  let canvas = document.select("canvas")?.cast::<HtmlCanvasElement>()?;

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
    Computer::run(&display, textarea_clone.value().split_whitespace()).unwrap();
  }) as Box<dyn FnMut()>);

  textarea
    .add_event_listener_with_callback("input", &cb.as_ref().unchecked_ref())
    .unwrap();

  cb.forget();

  web_sys::console::log_1(&"hello".into());

  Ok(())
}
