use super::*;

#[wasm_bindgen]
pub fn test(program: &str) -> Result<(), String> {
  test_inner(program).map_err(|err| err.to_string())
}

fn test_inner(program: &str) -> Result {
  let window = window();

  let canvas = window
    .get_document()
    .create_element("canvas")
    .map_err(JsValueError)?
    .cast::<HtmlCanvasElement>()?;

  canvas.set_height(256);
  canvas.set_width(256);

  let gpu = Arc::new(Mutex::new(Gpu::new(&canvas)?));

  let worker = Worker::new("/worker.js").map_err(JsValueError)?;

  worker.add_event_listener_with_event("message", move |event| {
    let state: State = serde_json::from_str(&event.data().as_string().unwrap()).unwrap();
    gpu.lock().unwrap().apply(&state).unwrap();
    gpu.lock().unwrap().render_to_canvas().unwrap();
    js_sys::eval("window.dataURL = document.getElementsByTagName('canvas')[0].toDataURL()").unwrap();
  })?;

  worker
    .post_message(&wasm_bindgen::JsValue::from_str(&serde_json::to_string(
      &Message {
        message_type: MessageType::Script,
        payload: Some(program),
      },
    )?))
    .map_err(JsValueError)?;

  worker
    .post_message(&wasm_bindgen::JsValue::from_str(&serde_json::to_string(
      &Message {
        message_type: MessageType::Run,
        payload: None,
      },
    )?))
    .map_err(JsValueError)?;

  Ok(())
}
