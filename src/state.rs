use {super::*, num_traits::identities::Zero};

pub(crate) struct State {
  matrix: DMatrix<Vector3<u8>>,
}

impl State {
  pub(crate) fn new() -> Self {
    Self {
      matrix: DMatrix::zeros(0, 0),
    }
  }

  pub(crate) fn resize(&mut self, dim: Vector2<usize>) {
    self.matrix.resize_mut(dim.x, dim.y, Zero::zero())
  }

  pub(crate) fn dimensions(&self) -> Vector2<usize> {
    Vector2::new(self.matrix.ncols(), self.matrix.nrows())
  }

  pub fn matrix(&mut self) -> &mut DMatrix<Vector3<u8>> {
    &mut self.matrix
  }

  pub(crate) fn image(&self) -> Result<RgbImage> {
    ImageBuffer::from_raw(
      self.matrix.nrows().try_into()?,
      self.matrix.ncols().try_into()?,
      self.matrix.transpose().iter().flatten().cloned().collect(),
    )
    .ok_or_else(|| "State is not a valid image".into())
  }

  pub(crate) fn write(&self) -> Result<()> {
    for (i, row) in self.matrix.row_iter().enumerate() {
      if i > 0 {
        println!();
      }
      for element in &row {
        if element.is_zero() {
          print!("0");
        } else {
          print!("1");
        }
      }
    }

    Ok(())
  }

  pub(crate) fn save(&self, path: PathBuf) -> Result<()> {
    self.image()?.save(path)?;

    Ok(())
  }
}
