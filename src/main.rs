use {
  crate::{command::Command, filter::Filter, operation::Operation, state::State},
  image::{ImageBuffer, RgbImage},
  nalgebra::{DMatrix, Vector2, Vector3},
  rand::Rng,
  std::{
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
    str::FromStr,
  },
};

mod command;
mod filter;
mod operation;
mod state;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() -> Result<()> {
  State::run()
}
