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

#[cfg(target_arch = "wasm32")]
use crate::browser::{display::Display, run};

#[cfg(not(target_arch = "wasm32"))]
use crate::native::{display::Display, run};

#[cfg(target_arch = "wasm32")]
mod browser;
mod color_axis;
mod command;
mod computer;
mod coordinates;
mod mask;
#[cfg(not(target_arch = "wasm32"))]
mod native;
mod operation;
mod pixel;
mod wrap;

type Error = Box<dyn std::error::Error>;
type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() {
  run();
}
