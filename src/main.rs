use {
  crate::{
    command::Command, coordinates::Coordinates, filter::Filter, operation::Operation, pixel::Pixel,
    state::State,
  },
  dirs::home_dir,
  image::{ImageBuffer, RgbImage},
  nalgebra::{DMatrix, Rotation2, Rotation3, Vector2, Vector3},
  rand::Rng,
  rustyline::{error::ReadlineError, Editor},
  std::{
    f64,
    io::{self, BufWriter, Write},
    path::{Path, PathBuf},
    process,
    str::FromStr,
  },
  strum::EnumString
};

mod command;
mod coordinates;
mod filter;
mod operation;
mod pixel;
mod state;

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
