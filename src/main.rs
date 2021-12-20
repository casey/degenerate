use {
  crate::{arguments::Arguments, filter::Filter, state::State},
  image::{io::Reader as ImageReader, ImageBuffer, RgbImage},
  nalgebra::{DMatrix, Vector2, Vector3},
  std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
    str::FromStr,
  },
  structopt::StructOpt,
};

mod arguments;
mod filter;
mod state;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() -> Result<()> {
  Arguments::from_args().run()
}
