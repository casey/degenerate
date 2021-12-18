use {
  crate::{arguments::Arguments, filter::Filter, state::State},
  image::{ImageBuffer, RgbImage},
  nalgebra::{DMatrix, Vector2, Vector3},
  std::{path::PathBuf, str::FromStr},
  structopt::StructOpt,
};

mod arguments;
mod filter;
mod state;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() -> Result<()> {
  let arguments = Arguments::from_args();

  let mut state = State::new();

  for filter in arguments.filters {
    filter.apply(&mut state);
  }

  if let Some(path) = arguments.output {
    state.save(path)?;
  } else {
    state.write()?;
  }

  Ok(())
}
