use super::*;

#[wasm_bindgen]
pub fn test(program: &str) -> Result<String, String> {
  test_inner(program).map_err(|err| err.to_string())
}

fn test_inner(program: &str) -> Result<String> {
  let program = Command::parse_program(program)?;

  let window = window();

  let canvas = window
    .get_document()
    .create_element("canvas")
    .map_err(JsValueError)?
    .cast::<HtmlCanvasElement>()?;

  canvas.set_height(256);
  canvas.set_width(256);

  let gpu = if window.location().hash().map_err(JsValueError)? == "#gpu" {
    Some(Arc::new(Mutex::new(Gpu::new(&canvas)?)))
  } else {
    None
  };

  let mut computer = Computer::new(gpu.clone());
  computer.load_program(&program);
  computer.resize(canvas.width().try_into()?)?;
  computer.run(false)?;

  if let Some(gpu) = gpu {
    gpu.lock().unwrap().render_to_canvas()?;
  } else {
    let pixels = computer
      .memory()
      .transpose()
      .iter()
      .flatten()
      .cloned()
      .collect::<Vec<u8>>();

    let image_data = ImageData::new_with_u8_clamped_array(
      wasm_bindgen::Clamped(&pixels),
      computer.memory().ncols().try_into()?,
    )
    .map_err(JsValueError)?;

    let context = canvas
      .get_context("2d")
      .map_err(JsValueError)?
      .ok_or("failed to retrieve context")?
      .cast::<CanvasRenderingContext2d>()?;

    context
      .put_image_data(&image_data, 0.0, 0.0)
      .map_err(JsValueError)?;
  }

  Ok(canvas.to_data_url().map_err(JsValueError)?)
}
