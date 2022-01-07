use {
  crate::{
    color_axis::ColorAxis, command::Command, coordinates::Coordinates, mask::Mask,
    operation::Operation, path_ext::PathExt, pixel::Pixel, state::State, wrap::Wrap,
  },
  dirs::home_dir,
  image::{ImageBuffer, RgbImage},
  nalgebra::{DMatrix, Rotation3, Similarity2, UnitComplex, Vector2, Vector3},
  rand::Rng,
  rand::{rngs::StdRng, SeedableRng},
  rustyline::{error::ReadlineError, Editor},
  std::{
    env, f64,
    io::{self, BufWriter, Write},
    path::{Path, PathBuf},
    process,
    str::FromStr,
  },
  strum::EnumString,
};

mod color_axis;
mod command;
mod coordinates;
mod mask;
mod operation;
mod path_ext;
mod pixel;
mod state;
mod wrap;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() {
  if let Err(error) = State::run() {
    if let Some(ReadlineError::Eof | ReadlineError::Interrupted) =
      error.downcast_ref::<ReadlineError>()
    {
      return;
    }

    eprintln!("error: {}", error);
    process::exit(1);
  }
}
