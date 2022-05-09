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
fn main() {
  use {
    wasm_bindgen::{closure::Closure, JsCast, JsValue},
    web_sys::{HtmlTextAreaElement, InputEvent},
  };

  fn render(event: InputEvent) -> Result<(), JsValue> {
    let text_area = event
      .current_target()
      .unwrap()
      .dyn_into::<HtmlTextAreaElement>()
      .unwrap();

    Computer::run(text_area.value().split_whitespace()).map_err(|err| format!("error: {err}"))?;

    Ok(())
  }

  let window = web_sys::window().unwrap();

  let document = window.document().unwrap();

  let cb = Closure::wrap(
    Box::new(|event: InputEvent| render(event)) as Box<dyn FnMut(_) -> Result<(), JsValue>>
  );

  document
    .get_elements_by_tag_name("textarea")
    .item(0)
    .unwrap()
    .dyn_into::<HtmlTextAreaElement>()
    .unwrap()
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
