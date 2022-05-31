use super::*;

#[wasm_bindgen]
pub fn test(program: &str) -> Result<String, String> {
  test_inner(program).map_err(|err| err.to_string())
}

fn test_inner(program: &str) -> Result<String> {
  // let program = Command::parse_program(program)?;

  let window = window();

  let canvas = window
    .get_document()
    .create_element("canvas")
    .map_err(JsValueError)?
    .cast::<HtmlCanvasElement>()?;

  canvas.set_height(256);
  canvas.set_width(256);

  let gpu = Arc::new(Mutex::new(Gpu::new(&canvas)?));

  let mut computer = Computer::new(gpu.clone());
  computer.load_program(program.into());
  computer.resize()?;
  // computer.run(false)?;
  gpu.lock().unwrap().render_to_canvas()?;

  Ok(canvas.to_data_url().map_err(JsValueError)?)
}
