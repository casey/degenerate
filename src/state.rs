use {
  super::*,
  num_traits::identities::Zero,
  rand::{rngs::StdRng, SeedableRng},
  std::env,
};

pub(crate) struct State {
  pub(crate) matrix: DMatrix<Vector3<u8>>,
  pub(crate) operation: Operation,
  pub(crate) rng: StdRng,
}

impl State {
  pub(crate) fn run() -> Result<()> {
    let commands = env::args()
      .skip(1)
      .map(|arg| arg.parse())
      .collect::<Result<Vec<Command>>>()?;

    let mut state = Self::new();

    for command in commands {
      command.apply(&mut state)?;
    }

    Ok(())
  }

  pub(crate) fn new() -> Self {
    Self {
      matrix: DMatrix::zeros(0, 0),
      operation: Operation::Invert,
      rng: StdRng::seed_from_u64(0),
    }
  }

  pub(crate) fn resize(&mut self, dim: Vector2<usize>) {
    self.matrix.resize_mut(dim.x, dim.y, Zero::zero())
  }

  pub(crate) fn dimensions(&self) -> Vector2<usize> {
    Vector2::new(self.matrix.ncols(), self.matrix.nrows())
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
}
