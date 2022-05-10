use {
  super::*,
  wasm_bindgen::{closure::Closure, JsCast},
  web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlTextAreaElement},
};

pub(crate) mod display;

// todo:
// - remove unwraps
// - surface errors to user
// - decide canvas layout

pub(crate) fn run() {
  console_error_panic_hook::set_once();

  let window = web_sys::window().unwrap();

  let document = window.document().unwrap();

  let textarea = document
    .query_selector("textarea")
    .unwrap()
    .unwrap()
    .dyn_into::<HtmlTextAreaElement>()
    .unwrap();

  let canvas = document
    .query_selector("canvas")
    .unwrap()
    .unwrap()
    .dyn_into::<HtmlCanvasElement>()
    .unwrap();

  let context = canvas
    .get_context("2d")
    .unwrap()
    .unwrap()
    .dyn_into::<CanvasRenderingContext2d>()
    .unwrap();

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
}
