use {
  super::*,
  wasm_bindgen::{closure::Closure, JsCast},
  web_sys::{
    CanvasRenderingContext2d, Document, Element, HtmlCanvasElement, HtmlTextAreaElement, Node,
    Window,
  },
};

// todo:
// - hide <main> somehow
// - surface errors to user
// - deploy to pages
// - remove unwraps
// - hide textarea outline
// - semantic error element

pub(crate) mod display;

pub(crate) fn run() {
  if let Err(err) = run_inner() {
    set_error(err);
  }
}

fn set_error(err: impl AsRef<dyn std::error::Error>) {
  window()
    .get_document()
    .select("#stderr")
    .unwrap()
    .cast::<Node>()
    .unwrap()
    .set_text_content(Some(&err.as_ref().to_string()));
}

fn clear_error() {
  window()
    .get_document()
    .select("#stderr")
    .unwrap()
    .cast::<Node>()
    .unwrap()
    .set_text_content(None)
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

trait WindowDocument {
  fn get_document(&self) -> Document;
}

impl WindowDocument for Window {
  fn get_document(&self) -> Document {
    self.document().expect("`window.document` missing")
  }
}

fn window() -> Window {
  web_sys::window().expect("`window` missing")
}

fn run_inner() -> Result {
  console_error_panic_hook::set_once();

  let window = window();

  let document = window.get_document();

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
    match Computer::run(&display, textarea_clone.value().split_whitespace()) {
      Err(err) => set_error(err),
      Ok(()) => clear_error(),
    }
  }) as Box<dyn FnMut()>);

  textarea
    .add_event_listener_with_callback("input", &cb.as_ref().unchecked_ref())
    .unwrap();

  cb.forget();

  Ok(())
}
