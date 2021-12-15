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

  pub(crate) fn generate(&mut self, width: usize, height: usize) {
    self.matrix.resize_mut(width, height, [0, 0, 0, 0]);
  }

  pub(crate) fn write(&self) -> Result<()> {
    let encoder =
      PnmEncoder::new(io::stdout()).with_subtype(PnmSubtype::Pixmap(SampleEncoding::Ascii));

    let mut buffer: Vec<u8> = Vec::with_capacity(self.matrix.len() * 4);
    for row in self.matrix.row_iter() {
      for pixel in &row {
        buffer.extend_from_slice(&pixel[..3]);
      }
    }

    encoder.write_image(
      &buffer,
      self.matrix.ncols() as u32,
      self.matrix.nrows() as u32,
      image::ColorType::Rgb8,
    )?;

    Ok(())
  }
}
