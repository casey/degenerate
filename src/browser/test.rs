use super::*;

#[wasm_bindgen]
pub fn test(program: &str) -> Result<String, String> {
  test_inner(program).map_err(|err| err.to_string())
}

fn test_inner(program: &str) -> Result<String> {
  let program = program
    .split_whitespace()
    .into_iter()
    .map(Command::from_str)
    .collect::<Result<Vec<Command>>>()?;

  let mut computer = Computer::new();
  computer.load_program(&program);
  computer.resize((256, 256));
  computer.run(false)?;

  let window = window();

  let document = window.get_document();

  let canvas = document
    .create_element("canvas")
    .map_err(JsValueError)?
    .cast::<HtmlCanvasElement>()?;

  canvas.set_height(256);
  canvas.set_width(256);

  let context = canvas
    .get_context("2d")
    .map_err(JsValueError)?
    .ok_or("failed to retrieve context")?
    .cast::<CanvasRenderingContext2d>()?;

  let mut pixels = Vec::new();

  for pixel in &computer.memory().transpose() {
    pixels.extend_from_slice(&[pixel.x, pixel.y, pixel.z, 255]);
  }

  let image_data = ImageData::new_with_u8_clamped_array(
    wasm_bindgen::Clamped(&pixels),
    computer.memory().ncols().try_into()?,
  )
  .map_err(JsValueError)?;

  context
    .put_image_data(&image_data, 0.0, 0.0)
    .map_err(JsValueError)?;

  Ok(canvas.to_data_url().map_err(JsValueError)?)
}
