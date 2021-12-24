use {
  crate::{command::Command, filter::Filter, operation::Operation, state::State},
  dirs::home_dir,
  image::{ImageBuffer, RgbImage},
  nalgebra::{DMatrix, Vector3},
  rand::Rng,
  rustyline::{error::ReadlineError, Editor},
  std::{
    io::{self, BufWriter, Write},
    path::{Path, PathBuf},
    process,
    str::FromStr,
  },
};

mod command;
mod filter;
mod operation;
mod state;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() {
  if let Err(error) = State::run() {
    if let Some(ReadlineError::Eof | ReadlineError::Interrupted) =
      error.downcast_ref::<ReadlineError>()
    {
    } else {
      eprintln!("error: {}", error);
      process::exit(1);
    }
  }
}
