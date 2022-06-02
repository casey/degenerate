use super::*;

pub(crate) struct App {
  animation_frame_callback: Option<Closure<dyn FnMut(f64)>>,
  animation_frame_pending: bool,
  canvas: HtmlCanvasElement,
  gpu: Arc<Mutex<Gpu>>,
  input: bool,
  nav: HtmlElement,
  program: String,
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

    let textarea = document.select("textarea")?.cast::<HtmlTextAreaElement>()?;

    let canvas = document.select("canvas")?.cast::<HtmlCanvasElement>()?;

    let nav = document.select("nav")?.cast::<HtmlElement>()?;

    let stderr = Stderr::get();

    let gpu = Arc::new(Mutex::new(Gpu::new(&canvas)?));

    let worker = Worker::new("/worker.js").map_err(JsValueError)?;

    let app = Arc::new(Mutex::new(Self {
      animation_frame_callback: None,
      animation_frame_pending: false,
      canvas: canvas.clone(),
      gpu,
      input: false,
      nav,
      program: String::new(),
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

    worker.add_event_listener_with_event("message", move |event| {
      let mut app = app.lock().unwrap();
      let event: WorkerEvent = serde_json::from_str(&event.data().as_string().unwrap()).unwrap();
      match event {
        WorkerEvent::Apply(state) => {
          app.gpu.lock().unwrap().apply(&state).unwrap();
          stderr.update(app.gpu.lock().unwrap().render_to_canvas());
          app.request_animation_frame().unwrap();
        }
        WorkerEvent::Done => {
          canvas.set_class_name("done");
        }
      }
    })?;

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
        .post_message(&wasm_bindgen::JsValue::from_str(&serde_json::to_string(
          &Message {
            message_type: MessageType::Script,
            payload: Some(&self.textarea.value()),
          },
        )?))
        .map_err(JsValueError)?;

      let program = self.textarea.value();

      log::trace!("Program: {:?}", program);

      let program_changed = program != self.program;

      if resize || program_changed {
        self.program = program;

        self.gpu.lock().unwrap().resize()?;

        self
          .worker
          .post_message(&wasm_bindgen::JsValue::from_str(&serde_json::to_string(
            &Message {
              message_type: MessageType::Run,
              payload: None,
            },
          )?))
          .map_err(JsValueError)?;
      }
    }

    Ok(())
  }
}
