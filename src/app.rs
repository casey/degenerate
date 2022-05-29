use super::*;

pub(crate) struct App {
  animation_frame_callback: Option<Closure<dyn FnMut(f64)>>,
  animation_frame_pending: bool,
  canvas: HtmlCanvasElement,
  computer: Computer,
  input: bool,
  nav: HtmlElement,
  resize: bool,
  stderr: Stderr,
  textarea: HtmlTextAreaElement,
  gpu: Arc<Mutex<Gpu>>,
  window: Window,
  audio_context: AudioContext,
}

impl App {
  pub(super) fn init() -> Result {
    let window = window();

    let document = window.get_document();

    let audio_element = document.select("audio")?.cast::<HtmlAudioElement>()?;
    let audio_context = AudioContext::new().map_err(JsValueError)?;
    let audio_analyzer = audio_context.create_analyser().map_err(JsValueError)?;
    let source = audio_context
      .create_media_element_source(&audio_element)
      .map_err(JsValueError)?;
    source
      .connect_with_audio_node(&audio_analyzer)
      .map_err(JsValueError)?;
    source
      .connect_with_audio_node(&audio_context.destination())
      .map_err(JsValueError)?;

    let textarea = document.select("textarea")?.cast::<HtmlTextAreaElement>()?;

    let canvas = document.select("canvas")?.cast::<HtmlCanvasElement>()?;

    let nav = document.select("nav")?.cast::<HtmlElement>()?;

    let stderr = Stderr::get();

    let gpu = Arc::new(Mutex::new(Gpu::new(&canvas, audio_analyzer)?));

    let app = Arc::new(Mutex::new(Self {
      animation_frame_callback: None,
      animation_frame_pending: false,
      canvas,
      computer: Computer::new(gpu.clone()),
      input: false,
      nav,
      resize: true,
      stderr: stderr.clone(),
      textarea: textarea.clone(),
      gpu,
      window: window.clone(),
      audio_context,
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

      self.canvas.set_height(device_pixel_height.ceil() as u32);
      self.canvas.set_width(device_pixel_width.ceil() as u32);

      self.resize = false;
    }

    if self.input {
      self.nav.set_class_name("fade-out");
      self.audio_context.resume();

      let program = Command::parse_program(&self.textarea.value())?;

      log::trace!("Program: {:?}", program);

      let program_changed = program != self.computer.program();

      if resize || program_changed {
        let mut computer = Computer::new(self.gpu.clone());
        computer.load_program(&program);
        computer.resize()?;
        self.computer = computer;
      }

      let run = !self.computer.done();

      if run {
        self.computer.run(true)?;
      }

      if resize || program_changed || run {
        self.gpu.lock().unwrap().render_to_canvas()?;

        if !self.computer.done() {
          self.request_animation_frame()?;
        }
      }
    }

    Ok(())
  }
}
