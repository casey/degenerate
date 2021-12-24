use {
  super::*,
  num_traits::identities::Zero,
  rand::{rngs::StdRng, SeedableRng},
  std::env,
};

pub(crate) struct State {
  pub(crate) loop_counter: usize,
  pub(crate) matrix: DMatrix<Vector3<u8>>,
  pub(crate) operation: Operation,
  pub(crate) program: Vec<Command>,
  pub(crate) program_counter: usize,
  pub(crate) rng: StdRng,
  pub(crate) verbose: bool,
}

impl State {
  pub(crate) fn run() -> Result<()> {
    let mut state = Self::new();

    for arg in env::args().skip(1) {
      state.program.push(arg.parse()?);
    }

    while let Some(command) = state.program.get(state.program_counter).cloned() {
      if state.verbose {
        eprintln!(
          "PC {} LC {} {:?}",
          state.program_counter, state.loop_counter, command
        );
      }
      command.apply(&mut state)?;
      state.program_counter = state.program_counter.wrapping_add(1);
    }

    Ok(())
  }

  pub(crate) fn new() -> Self {
    Self {
      loop_counter: 0,
      matrix: DMatrix::zeros(0, 0),
      operation: Operation::Invert,
      program: Vec::new(),
      program_counter: 0,
      rng: StdRng::seed_from_u64(0),
      verbose: false,
    }
  }

  pub(crate) fn resize(&mut self, dimensions: (usize, usize)) {
    self
      .matrix
      .resize_mut(dimensions.0, dimensions.1, Zero::zero())
  }

  pub(crate) fn image(&self) -> Result<RgbImage> {
    ImageBuffer::from_raw(
      self.matrix.ncols().try_into()?,
      self.matrix.nrows().try_into()?,
      self.matrix.transpose().iter().flatten().cloned().collect(),
    )
    .ok_or_else(|| "State is not a valid image".into())
  }

  pub(crate) fn print(&self) -> Result<()> {
    let mut w = BufWriter::new(io::stdout());

    for row in self.matrix.row_iter() {
      for element in &row {
        write!(
          w,
          "{:X}",
          element.map(|scalar| scalar as u32).sum() / (16 * 3)
        )?;
      }
      writeln!(w)?;
    }

    w.flush()?;

    Ok(())
  }

  pub(crate) fn load(&mut self, path: &Path) -> Result<()> {
    let image = image::io::Reader::open(path)?
      .decode()?
      .as_rgb8()
      .ok_or_else(|| format!("{} is not a valid rgb8 image", path.display()))?
      .to_owned();

    let (width, height) = (image.width() as usize, image.height() as usize);

    self.matrix = DMatrix::from_iterator(
      height,
      width,
      image
        .rows()
        .map(|row| row.map(|pixel| Vector3::new(pixel[0], pixel[1], pixel[2])))
        .flatten(),
    );

    Ok(())
  }
}
