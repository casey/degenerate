use super::*;

pub(crate) struct State {
  matrix: DMatrix<[u8; 4]>,
}

impl State {
  pub(crate) fn new() -> Self {
    Self {
      matrix: DMatrix::repeat(0, 0, [0, 0, 0, 0]),
    }
  }

  pub(crate) fn width(&self) -> usize {
    self.matrix.ncols()
  }

  pub(crate) fn height(&self) -> usize {
    self.matrix.nrows()
  }

  pub(crate) fn dimensions(&self) -> (usize, usize) {
    (self.width(), self.height())
  }

  pub fn matrix(&mut self) -> &mut DMatrix<[u8; 4]> {
    &mut self.matrix
  }

  pub(crate) fn scalars(&self) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(self.matrix.len() * 4);

    for row in self.matrix.row_iter() {
      for pixel in &row {
        buffer.extend_from_slice(&pixel[..3]);
      }
    }

    buffer
  }

  pub(crate) fn generate(&mut self, width: usize, height: usize) {
    self.matrix.resize_mut(width, height, [0, 0, 0, 0]);
  }

  pub(crate) fn write(&self) -> Result<()> {
    let encoder =
      PnmEncoder::new(io::stdout()).with_subtype(PnmSubtype::Pixmap(SampleEncoding::Ascii));

    encoder.write_image(
      &self.scalars(),
      self.matrix.ncols() as u32,
      self.matrix.nrows() as u32,
      image::ColorType::Rgb8,
    )?;

    Ok(())
  }

  pub(crate) fn save(&self, path: PathBuf) -> Result<()> {
    let image: Result<RgbImage> =
      ImageBuffer::from_raw(self.width() as u32, self.height() as u32, self.scalars())
        .ok_or_else(|| "State is not a valid image".into());

    image?.save(path)?;

    Ok(())
  }
}
