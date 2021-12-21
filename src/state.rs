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

  pub(crate) fn write(&self, w: impl Write) -> Result<()> {
    let mut w = BufWriter::new(w);

    for row in self.matrix.row_iter() {
      for element in &row {
        if element.is_zero() {
          write!(w, "0")?;
        } else {
          write!(w, "1")?;
        }
      }
      writeln!(w)?;
    }

    w.flush()?;

    Ok(())
  }

  pub(crate) fn load(&mut self, path: &PathBuf) -> Result<()> {
    match path.extension() {
      Some(ext) if ext == "txt" => {
        let content = fs::read_to_string(&path)?;

        let lines = content.lines();

        let width = lines
          .clone()
          .collect::<Vec<&str>>()
          .first()
          .unwrap_or(&"")
          .len();

        let height = lines.clone().count();

        self.matrix = DMatrix::from_iterator(
          width,
          height,
          lines
            .map(|line| {
              line
                .chars()
                .map(|c| Vector3::from_element(c.to_digit(2).unwrap_or(0) as u8))
            })
            .flatten(),
        );
      }
      _ => {
        let image = ImageReader::open(path)?.decode()?;

        if let Some(image) = image.as_rgb8() {
          let (width, height) = (image.width() as usize, image.height() as usize);

          self.matrix = DMatrix::from_iterator(
            height,
            width,
            image
              .rows()
              .map(|row| row.map(|pixel| Vector3::new(pixel[0], pixel[1], pixel[2])))
              .flatten(),
          );
        }
      }
    }

    Ok(())
  }

  pub(crate) fn save(&self, path: &PathBuf) -> Result<()> {
    match path.extension() {
      Some(ext) if ext == "txt" => self.write(File::create(path)?),
      _ => Ok(self.image()?.save(path)?),
    }
  }
}
