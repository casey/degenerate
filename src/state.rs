use {
  super::*,
  num_traits::identities::Zero,
  rand::{rngs::StdRng, SeedableRng},
};

pub(crate) struct State {
  pub(crate) matrix: DMatrix<Vector3<u8>>,
  pub(crate) operation: Operation,
  pub(crate) rng: StdRng,
}

impl State {
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

  pub(crate) fn write(&self, w: impl Write) -> Result<()> {
    let mut w = BufWriter::new(w);

    for row in self.matrix.row_iter() {
      for element in &row {
        write!(w, "{:X}", element.map(|scalar| scalar as u32).sum() / 48)?;
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

  pub(crate) fn save(&self, path: &Path) -> Result<()> {
    match path.extension() {
      Some(ext) if ext == "txt" => self.write(File::create(path)?),
      _ => Ok(self.image()?.save(path)?),
    }
  }
}
