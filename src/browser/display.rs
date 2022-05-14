use super::*;

use web_sys::ImageData;

pub(crate) struct Display {
  pub(super) context: CanvasRenderingContext2d,
}

impl Display {
  pub(crate) fn render(&self, memory: &DMatrix<Vector3<u8>>) -> Result<()> {
    let mut pixels = Vec::new();

    for pixel in &memory.transpose() {
      pixels.extend_from_slice(&[pixel.x, pixel.y, pixel.z, 255]);
    }

    let image_data = ImageData::new_with_u8_clamped_array(
      wasm_bindgen::Clamped(&pixels),
      memory.ncols().try_into()?,
    )
    .map_err(JsValueError)?;

    self
      .context
      .put_image_data(&image_data, 0.0, 0.0)
      .map_err(JsValueError)?;

    Ok(())
  }

  pub(crate) fn dimensions(&self) -> Result<(usize, usize)> {
    let canvas = self
      .context
      .canvas()
      .ok_or_else(|| "failed to get context canvas".to_string())?;

    Ok((canvas.height().try_into()?, canvas.width().try_into()?))
  }
}
