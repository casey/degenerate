use {
  crate::{arguments::Arguments, filter::Filter, state::State},
  image::{
    pnm::{PnmEncoder, PnmSubtype, SampleEncoding},
    ImageEncoder,
  },
  std::{io, path::PathBuf, slice::ChunksMut, str::FromStr},
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

  // state.image()?.save("output.png")?;

  let encoder =
    PnmEncoder::new(io::stdout()).with_subtype(PnmSubtype::Pixmap(SampleEncoding::Ascii));

  encoder.write_image(
    state.scalars(),
    state.width().try_into()?,
    state.height().try_into()?,
    image::ColorType::Rgb8,
  )?;

  Ok(())
}
