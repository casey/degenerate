use super::*;

pub(crate) struct Display {
  pub(super) context: CanvasRenderingContext2d,
}

impl Display {
  pub(crate) fn render(&self, memory: &DMatrix<Vector3<u8>>) {
    let mut pixels = Vec::new();

    for pixel in &memory.transpose() {
      pixels.extend_from_slice(&[pixel.x, pixel.y, pixel.z, 255]);
    }

    let image_data = web_sys::ImageData::new_with_u8_clamped_array(
      wasm_bindgen::Clamped(&pixels),
      memory.ncols().try_into().unwrap(),
    )
    .unwrap();

    self.context.put_image_data(&image_data, 0.0, 0.0).unwrap();
  }

  pub(crate) fn dimensions(&self) -> (usize, usize) {
    let canvas = self.context.canvas().unwrap();

    (
      canvas.height().try_into().unwrap(),
      canvas.width().try_into().unwrap(),
    )
  }
}
