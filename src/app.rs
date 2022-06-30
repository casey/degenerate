use super::*;

pub(crate) struct App {
  animation_frame_callback: Option<Closure<dyn FnMut(f64)>>,
  audio_context: AudioContext,
  analyser_node: AnalyserNode,
  document: Document,
  gpu: Gpu,
  html: HtmlElement,
  nav: HtmlElement,
  select: HtmlSelectElement,
  stderr: Stderr,
  textarea: HtmlTextAreaElement,
  window: Window,
  worker: Worker,
  recording: bool,
}

impl App {
  pub(super) fn init() -> Result {
    let window = window();

    let document = window.get_document();

    let html = document.select("html")?.cast::<HtmlElement>()?;

    let textarea = document.select("textarea")?.cast::<HtmlTextAreaElement>()?;

    let canvas = document.select("canvas")?.cast::<HtmlCanvasElement>()?;

    let nav = document.select("nav")?.cast::<HtmlElement>()?;

    let select = document.select("select")?.cast::<HtmlSelectElement>()?;

    let examples = &[("all", include_str!("../examples/all.js"))];

    for (name, program) in examples {
      let option = document
        .create_element("option")
        .map_err(JsValueError)?
        .cast::<HtmlOptionElement>()?;

      option.set_text(name);

      option.set_value(program);

      select
        .add_with_html_option_element(&option)
        .map_err(JsValueError)?;
    }

    let stderr = Stderr::get();

    let audio_context = AudioContext::new().map_err(JsValueError)?;

    let analyser_node = audio_context.create_analyser().map_err(JsValueError)?;

    let gpu = Gpu::new(&window, &canvas, &analyser_node)?;

    let worker = Worker::new("/worker.js").map_err(JsValueError)?;

    let app = Arc::new(Mutex::new(Self {
      animation_frame_callback: None,
      analyser_node,
      audio_context,
      document,
      gpu,
      html,
      nav,
      select: select.clone(),
      stderr,
      textarea: textarea.clone(),
      window,
      worker: worker.clone(),
      recording: false,
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
      let app = local.lock().unwrap();
      app.hide_nav();
      let _ = app.audio_context.resume().unwrap();
    })?;

    let local = app.clone();
    textarea.add_event_listener_with_event("keydown", move |event| {
      let mut app = local.lock().unwrap();
      let result = app.on_keydown(event);
      app.stderr.update(result);
    })?;

    let local = app.clone();
    select.add_event_listener("change", move || {
      let mut app = local.lock().unwrap();
      let result = app.on_selection_changed();
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
      WorkerMessage::Clear => {
        self.gpu.clear()?;
      }
      WorkerMessage::Done => {
        self.html.set_class_name("done");
      }
      WorkerMessage::Render(state) => {
        if state.operation == 4 /* OPERATION_SAMPLE */ && !self.recording {
          let audio_context = self.audio_context.clone();
          let analyser_node = self.analyser_node.clone();
          let closure = Closure::wrap(Box::new(move |stream: JsValue| {
            audio_context
              .create_media_stream_source(&stream.cast::<MediaStream>().unwrap())
              .unwrap()
              .connect_with_audio_node(&analyser_node)
              .map_err(JsValueError)
              .unwrap();
          }) as Box<dyn FnMut(JsValue)>);
          let _ = self
            .window
            .navigator()
            .media_devices()
            .map_err(JsValueError)?
            .get_user_media_with_constraints(
              MediaStreamConstraints::new()
                .audio(&true.into())
                .video(&false.into()),
            )
            .map_err(JsValueError)?
            .then(&closure);
          closure.forget();
          self.recording = true;
        }
        if state.oscillator_frequency > 0.0 && !self.recording {
          let audio_context = self.audio_context.clone();
          audio_context.resume().unwrap();
          // let analyser_node = self.analyser_node.clone();

          let oscillator_node = OscillatorNode::new_with_options(
            &audio_context,
            OscillatorOptions::new().frequency(440.0),
          )
          .unwrap();

          oscillator_node
            .connect_with_audio_node(&self.audio_context.destination())
            .unwrap();
          oscillator_node.start().unwrap();

          self.recording = true;
        }
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

  fn on_selection_changed(&mut self) -> Result {
    self.hide_nav();
    self.textarea.set_value(&self.select.value());
    self.textarea.focus().map_err(JsValueError)?;
    Ok(())
  }

  fn hide_nav(&self) {
    self.nav.set_class_name("fade-out");
  }
}
