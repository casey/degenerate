use {
  crate::{
    color_axis::ColorAxis, command::Command, computer::Computer, coordinates::Coordinates,
    mask::Mask, operation::Operation, pixel::Pixel, wrap::Wrap,
  },
  image::{ImageBuffer, RgbImage},
  nalgebra::{DMatrix, Rotation3, Similarity2, UnitComplex, Vector2, Vector3},
  rand::Rng,
  rand::{rngs::StdRng, SeedableRng},
  std::{
    env, f64, fs,
    io::{self, BufWriter, Write},
    path::{Path, PathBuf},
    process,
    str::FromStr,
  },
  strum::EnumString,
};

mod color_axis;
mod command;
mod computer;
mod coordinates;
mod mask;
mod operation;
mod pixel;
mod wrap;

type Error = Box<dyn std::error::Error>;
type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

#[cfg(target_arch = "wasm32")]
use {
  wasm_bindgen::{closure::Closure, JsCast, JsValue},
  web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlTextAreaElement},
};

#[cfg(target_arch = "wasm32")]
fn main() {
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

  struct Foo {
    textarea: HtmlTextAreaElement,
    context: CanvasRenderingContext2d,
  }

  impl Foo {
    fn render(&self) -> Result<(), JsValue> {
      Computer::run(self.textarea.value().split_whitespace(), &self.context)
        .map_err(|err| format!("error: {err}"))?;
      Ok(())
    }
  }

  let foo = Foo {
    textarea: textarea.clone(),
    context,
  };

  let cb = Closure::wrap(Box::new(move || foo.render()) as Box<dyn FnMut() -> Result<(), JsValue>>);

  textarea
    .add_event_listener_with_callback("input", &cb.as_ref().unchecked_ref())
    .unwrap();

  cb.forget();

  web_sys::console::log_1(&"hello".into());
}

#[cfg(not(target_arch = "wasm32"))]
use {
  ansi_term::{Colour::Red, Style},
  dirs::home_dir,
  rustyline::{error::ReadlineError, Editor},
};

#[cfg(not(target_arch = "wasm32"))]
fn main() {
  if let Err(error) = Computer::run(env::args().skip(1)) {
    if let Some(ReadlineError::Eof | ReadlineError::Interrupted) =
      error.downcast_ref::<ReadlineError>()
    {
      return;
    }

    if atty::is(atty::Stream::Stderr)
      || env::var("CLICOLOR_FORCE")
        .map(|val| val != "0")
        .unwrap_or_default()
    {
      eprintln!(
        "{}{}",
        Red.bold().paint("error"),
        Style::new().bold().paint(format!(": {}", error))
      );
    } else {
      eprintln!("error: {}", error);
    }

    process::exit(1);
  }
}
