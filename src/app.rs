use super::*;

pub(crate) struct App {
  animation_frame_callback: Option<Closure<dyn FnMut(f64)>>,
  animation_frame_pending: bool,
  canvas: HtmlCanvasElement,
  gpu: Gpu,
  input: bool,
  nav: HtmlElement,
  resize: bool,
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

    let gpu = Gpu::new(&canvas)?;

    let worker = Worker::new("/worker.js").map_err(JsValueError)?;

    let app = Arc::new(Mutex::new(Self {
      animation_frame_callback: None,
      animation_frame_pending: false,
      canvas,
      gpu,
      input: false,
      nav,
      resize: true,
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
    window.add_event_listener("resize", move || {
      let mut app = local.lock().unwrap();
      let result = app.on_resize();
      app.stderr.update(result);
    })?;

    let local = app.clone();
    textarea.add_event_listener("input", move || {
      let mut app = local.lock().unwrap();
      let result = app.on_input();
      app.stderr.update(result);
    })?;

    let local_html = html.clone();
    worker.add_event_listener_with_event("message", move |event| -> Result<(), String> {
      let mut app = app.lock().unwrap();

      let event: WorkerMessage = serde_json::from_str(
        &event
          .data()
          .as_string()
          .ok_or("Failed to retrieve event data as a string")?,
      )
      .map_err(|err| err.to_string())?;

      match event {
        WorkerMessage::Render(state) => {
          app.gpu.render(&state).map_err(|err| err.to_string())?;
          stderr.update(app.gpu.render_to_canvas());
          app
            .request_animation_frame()
            .map_err(|err| err.to_string())?;
        }
        WorkerMessage::Done => {
          local_html.set_class_name("done");
        }
      }

      Ok(())
    })?;

    html.set_class_name("ready");

    Ok(())
  }

  pub(super) fn on_resize(&mut self) -> Result {
    self.resize = true;
    self.request_animation_frame()?;
    Ok(())
  }

  pub(super) fn on_input(&mut self) -> Result {
    self.input = true;
    self.request_animation_frame()?;
    Ok(())
  }

  fn request_animation_frame(&mut self) -> Result {
    if self.animation_frame_pending {
      return Ok(());
    }

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

    self.animation_frame_pending = true;

    Ok(())
  }

  fn on_animation_frame(&mut self, timestamp: f64) -> Result {
    self.animation_frame_pending = false;

    log::trace!("Animation frame timestamp {}s", timestamp);

    let resize = self.resize;

    if self.resize {
      let css_pixel_height: f64 = self.canvas.client_height().try_into()?;
      let css_pixel_width: f64 = self.canvas.client_width().try_into()?;

      let device_pixel_ratio = self.window.device_pixel_ratio();
      let device_pixel_height = css_pixel_height * device_pixel_ratio;
      let device_pixel_width = css_pixel_width * device_pixel_ratio;

      self.canvas.set_height(device_pixel_height.ceil() as u32);
      self.canvas.set_width(device_pixel_width.ceil() as u32);

      self.resize = false;
    }

    if self.input {
      self.nav.set_class_name("fade-out");

      self
        .worker
        .post_message(&JsValue::from_str(&serde_json::to_string(
          &AppMessage::Script(&self.textarea.value()),
        )?))
        .map_err(JsValueError)?;

      let program = self.textarea.value();

      log::trace!("Program: {:?}", program);

      if resize {
        self.gpu.resize()?;

        self
          .worker
          .post_message(&JsValue::from_str(&serde_json::to_string(
            &AppMessage::Run,
          )?))
          .map_err(JsValueError)?;
      }
    }

    Ok(())
  }
}
