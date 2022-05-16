use super::*;

pub(crate) struct App {
  animation_frame_callback: Option<Closure<dyn FnMut(f64)>>,
  animation_frame_pending: bool,
  canvas: HtmlCanvasElement,
  computer: Computer,
  context: CanvasRenderingContext2d,
  input: bool,
  nav: HtmlElement,
  resize: bool,
  stderr: Stderr,
  textarea: HtmlTextAreaElement,
  window: Window,
}

impl App {
  pub(super) fn init() -> Result {
    let window = window();

    let document = window.get_document();

    let textarea = document.select("textarea")?.cast::<HtmlTextAreaElement>()?;

    let canvas = document.select("canvas")?.cast::<HtmlCanvasElement>()?;

    let nav = document.select("nav")?.cast::<HtmlElement>()?;

    let stderr = Stderr::get();

    let context = canvas
      .get_context("2d")
      .map_err(JsValueError)?
      .ok_or("failed to retrieve context")?
      .cast::<CanvasRenderingContext2d>()?;

    let app = Arc::new(Mutex::new(Self {
      animation_frame_callback: None,
      animation_frame_pending: false,
      context,
      canvas,
      input: false,
      nav,
      resize: true,
      stderr: stderr.clone(),
      textarea: textarea.clone(),
      window: window.clone(),
      computer: Computer::new(),
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

    textarea.add_event_listener("input", move || {
      let mut app = app.lock().unwrap();
      let result = app.on_input();
      stderr.update(result);
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
      let height = if cfg!(debug_assertions) {
        device_pixel_height / 32.0
      } else {
        device_pixel_height
      };
      let width = if cfg!(debug_assertions) {
        device_pixel_width / 32.0
      } else {
        device_pixel_width
      };
      self.canvas.set_height(height.ceil() as u32);
      self.canvas.set_width(width.ceil() as u32);
      self.resize = false;
    }

    if self.input {
      self.nav.set_class_name("fade-out");

      let program = self
        .textarea
        .value()
        .split_whitespace()
        .into_iter()
        .map(Command::from_str)
        .collect::<Result<Vec<Command>>>()?;

      log::trace!("Program: {:?}", program);

      let program_changed = program != self.computer.program();

      if resize || program_changed {
        let mut computer = Computer::new();
        computer.load_program(&program);

        computer.resize((
          self.canvas.height().try_into()?,
          self.canvas.width().try_into()?,
        ));

        self.computer = computer;
      }

      let done = self.computer.done();

      if !done {
        self.computer.run(true)?;
      }

      if resize || program_changed || !done {
        let mut pixels = Vec::new();

        for pixel in &self.computer.memory().transpose() {
          pixels.extend_from_slice(&[pixel.x, pixel.y, pixel.z, 255]);
        }

        let image_data = ImageData::new_with_u8_clamped_array(
          wasm_bindgen::Clamped(&pixels),
          self.computer.memory().ncols().try_into()?,
        )
        .map_err(JsValueError)?;

        self
          .context
          .put_image_data(&image_data, 0.0, 0.0)
          .map_err(JsValueError)?;

        if !self.computer.done() {
          self.request_animation_frame()?;
        }
      }
    }

    Ok(())
  }
}
