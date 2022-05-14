use super::*;

pub(crate) struct App {
  animation_frame_callback: Option<Closure<dyn FnMut(f64)>>,
  animation_frame_pending: bool,
  canvas: HtmlCanvasElement,
  display: Display,
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
      .map_err(|err| format!("`canvas.get_context(\"2d\")` failed: {:?}", err))?
      .ok_or_else(|| format!("failed to retrieve context"))?
      .cast::<CanvasRenderingContext2d>()?;

    let app = Arc::new(Mutex::new(Self {
      animation_frame_callback: None,
      animation_frame_pending: false,
      canvas,
      display: Display { context },
      input: false,
      nav,
      resize: true,
      stderr: stderr.clone(),
      textarea: textarea.clone(),
      window: window.clone(),
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
        &self
          .animation_frame_callback
          .as_ref()
          .unwrap()
          .as_ref()
          .dyn_ref()
          .unwrap(),
      )
      .map_err(|err| format!("`window.requestAnimationFrame` failed: {:?}", err))?;

    self.animation_frame_pending = true;

    Ok(())
  }

  fn on_animation_frame(&mut self, timestamp: f64) -> Result {
    self.animation_frame_pending = false;

    log::trace!("Animation frame timestamp {}s", timestamp);

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
      let program = self.textarea.value();
      log::trace!("Program: {}", program);
      Computer::run(&self.display, program.split_whitespace())?;
    }

    Ok(())
  }
}