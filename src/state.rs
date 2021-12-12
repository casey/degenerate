use super::*;

use image::{ImageBuffer, RgbImage};

pub(crate) struct State {
  width: usize,
  height: usize,
  buffer: Vec<u8>,
}

impl State {
  pub(crate) fn new() -> Self {
    Self {
      width: 0,
      height: 0,
      buffer: Vec::new(),
    }
  }

  pub(crate) fn dimensions(&self) -> (usize, usize) {
    (self.width, self.height)
  }

  pub(crate) fn height(&self) -> usize {
    self.height
  }

  pub(crate) fn width(&self) -> usize {
    self.width
  }

  pub(crate) fn get_pixel(&self, row: usize, col: usize) -> [u8; 3] {
    let i = (row * self.width + col) * 3;

    [self.buffer[i], self.buffer[i + 1], self.buffer[i + 2]]
  }

  pub(crate) fn set_pixel(&mut self, row: usize, col: usize, pixel: [u8; 3]) {
    let i = (row * self.width + col) * 3;

    self.buffer[i] = pixel[0];
    self.buffer[i + 1] = pixel[1];
    self.buffer[i + 2] = pixel[2];
  }

  pub(crate) fn generate(&mut self, width: usize, height: usize) {
    self.width = width;
    self.height = height;
    self.buffer = vec![0; width * height * 3];
  }

  pub(crate) fn scalars_mut(&mut self) -> &mut [u8] {
    &mut self.buffer
  }

  pub(crate) fn image(&self) -> Result<RgbImage> {
    ImageBuffer::from_raw(self.width as u32, self.height as u32, self.buffer.clone())
      .ok_or(format!("State is not valid image.").into())
  }
}
