use super::*;

pub(crate) struct App {
  animation_frame_callback: Option<Closure<dyn FnMut(f64)>>,
  document: Document,
  gpu: Gpu,
  html: HtmlElement,
  nav: HtmlElement,
  stderr: Stderr,
  textarea: HtmlTextAreaElement,
  window: Window,
  worker: Worker,
}

impl App {
  pub(super) fn init() -> Result {
    let window = window();

    let document = window.get_document();

    let html = document.select("html")?.cast::<HtmlElement>()?;

    let textarea = document.select("textarea")?.cast::<HtmlTextAreaElement>()?;

    let canvas = document.select("canvas")?.cast::<HtmlCanvasElement>()?;

    let nav = document.select("nav")?.cast::<HtmlElement>()?;

    let stderr = Stderr::get();

    let gpu = Gpu::new(&canvas, &window)?;

    let worker = Worker::new("/worker.js").map_err(JsValueError)?;

    let app = Arc::new(Mutex::new(Self {
      document,
      html,
      animation_frame_callback: None,
      gpu,
      nav,
      stderr: stderr.clone(),
      textarea: textarea.clone(),
      window: window.clone(),
      worker: worker.clone(),
    }));

    let local = app.clone();
    app.lock().unwrap().animation_frame_callback = Some(Closure::wrap(Box::new(move |timestamp| {
      let mut app = local.lock().unwrap();
      let result = app.on_animation_frame(timestamp);
      app.stderr.update(result);
    })
      as Box<dyn FnMut(f64)>));

    let local = app.clone();
    textarea.add_event_listener("input", move || {
      local.lock().unwrap().nav.set_class_name("fade-out");
    })?;

    let local = app.clone();
    textarea.add_event_listener_with_event("keydown", move |event| {
      let mut app = local.lock().unwrap();
      let result = app.on_keydown(event);
      app.stderr.update(result);
    })?;

    let local = app.clone();
    worker.add_event_listener_with_event("message", move |event: MessageEvent| {
      let mut app = local.lock().unwrap();
      let result = app.on_message(event);
      app.stderr.update(result);
    })?;

    let mut app = app.lock().unwrap();
    app.request_animation_frame()?;
    app.html.set_class_name("ready");

    Ok(())
  }

  pub(super) fn on_keydown(&mut self, event: KeyboardEvent) -> Result {
    if event.shift_key() && event.key() == "Enter" {
      event.prevent_default();
      self
        .worker
        .post_message(&JsValue::from_str(&serde_json::to_string(
          &AppMessage::Script(&self.textarea.value()),
        )?))
        .map_err(JsValueError)?;
    }
    Ok(())
  }

  fn request_animation_frame(&mut self) -> Result {
    self
      .window
      .request_animation_frame(
        self
          .animation_frame_callback
          .as_ref()
          .unwrap()
          .as_ref()
          .dyn_ref()
          .unwrap(),
      )
      .map_err(JsValueError)?;
    Ok(())
  }

  fn on_animation_frame(&mut self, timestamp: f64) -> Result {
    self.request_animation_frame()?;

    self.gpu.resize()?;

    log::trace!("Animation frame timestamp {}s", timestamp);

    self
      .worker
      .post_message(&JsValue::from_str(&serde_json::to_string(
        &AppMessage::Frame,
      )?))
      .map_err(JsValueError)?;

    Ok(())
  }

  fn on_message(&mut self, event: MessageEvent) -> Result {
    let event = serde_json::from_str(
      &event
        .data()
        .as_string()
        .ok_or("Failed to retrieve event data as a string")?,
    )?;

    match event {
      WorkerMessage::Done => {
        self.html.set_class_name("done");
      }
      WorkerMessage::Render(state) => {
        self.gpu.render(&state)?;
        self.gpu.present()?;
      }
      WorkerMessage::Resolution(resolution) => {
        self.gpu.lock_resolution(resolution);
      }
      WorkerMessage::Save => {
        let image = self.gpu.save_image()?;
        let mut png = Cursor::new(Vec::new());
        image.write_to(&mut png, ImageOutputFormat::Png)?;
        let a = self
          .document
          .create_element("a")
          .map_err(JsValueError)?
          .cast::<HtmlAnchorElement>()?;
        a.set_download("degenerate.png");
        let mut href = String::from("data:image/png;base64,");
        base64::encode_config_buf(png.get_ref(), base64::STANDARD, &mut href);
        a.set_href(&href);
        a.click();
      }
    }

    Ok(())
  }
}
