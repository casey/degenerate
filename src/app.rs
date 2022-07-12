use super::*;

pub(crate) struct App {
  analyser_node: AnalyserNode,
  animation_frame_callback: Option<Closure<dyn FnMut(f64)>>,
  audio_context: AudioContext,
  button: HtmlButtonElement,
  document: Document,
  gpu: Gpu,
  html: HtmlElement,
  nav: HtmlElement,
  oscillator_node: OscillatorNode,
  oscillator_gain_node: GainNode,
  select: HtmlSelectElement,
  stderr: Stderr,
  textarea: HtmlTextAreaElement,
  this: Option<Arc<Mutex<Self>>>,
  window: Window,
  worker: Worker,
  recording: bool,
}

lazy_static! {
  static ref EXAMPLES: BTreeMap<&'static str, &'static str> = [
    ("All", include_str!("../examples/all.js")),
    ("Kaleidoscope", include_str!("../examples/kaleidoscope.js")),
    ("Orb Zoom", include_str!("../examples/orb_zoom.js")),
    ("Orbs", include_str!("../examples/orbs.js")),
    ("Oscillator", include_str!("../examples/oscillator.js")),
    ("Pattern", include_str!("../examples/pattern.js")),
    ("Record", include_str!("../examples/record.js")),
    ("Starburst", include_str!("../examples/starburst.js")),
    ("Target", include_str!("../examples/target.js")),
    ("X", include_str!("../examples/x.js")),
  ]
  .iter()
  .cloned()
  .collect();
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

    let button = document.select("button")?.cast::<HtmlButtonElement>()?;

    for name in EXAMPLES.keys() {
      let option = document
        .create_element("option")
        .map_err(JsValueError)?
        .cast::<HtmlOptionElement>()?;

      option.set_text(name);

      option.set_value(name);

      select
        .add_with_html_option_element(&option)
        .map_err(JsValueError)?;
    }

    let stderr = Stderr::get();

    let audio_context = AudioContext::new().map_err(JsValueError)?;

    let analyser_node = audio_context.create_analyser().map_err(JsValueError)?;

    analyser_node.set_smoothing_time_constant(0.5);

    let gpu = Gpu::new(&window, &canvas, &analyser_node)?;

    let worker = Worker::new("/worker.js").map_err(JsValueError)?;

    let oscillator_gain_node = audio_context.create_gain().map_err(JsValueError)?;
    oscillator_gain_node.gain().set_value(0.0);

    let oscillator_node = audio_context.create_oscillator().map_err(JsValueError)?;
    oscillator_node.frequency().set_value(60.0);

    oscillator_node
      .connect_with_audio_node(&oscillator_gain_node)
      .map_err(JsValueError)?;
    oscillator_node.start().map_err(JsValueError)?;

    oscillator_gain_node
      .connect_with_audio_node(&audio_context.destination())
      .map_err(JsValueError)?;

    oscillator_gain_node
      .connect_with_audio_node(&analyser_node)
      .map_err(JsValueError)?;

    let app = Arc::new(Mutex::new(Self {
      analyser_node,
      animation_frame_callback: None,
      audio_context,
      button: button.clone(),
      document,
      gpu,
      html,
      nav,
      oscillator_gain_node,
      oscillator_node,
      recording: false,
      select: select.clone(),
      stderr,
      textarea: textarea.clone(),
      this: None,
      window,
      worker: worker.clone(),
    }));

    {
      let mut this = app.lock().unwrap();
      this.this = Some(app.clone());
    }

    {
      let local = app.clone();
      let mut this = app.lock().unwrap();
      this.animation_frame_callback = Some(Closure::wrap(Box::new(move |timestamp| {
        let mut app = local.lock().unwrap();
        let result = app.on_animation_frame(timestamp);
        app.stderr.update(result);
      }) as Box<dyn FnMut(f64)>));
    }

    let local = app.clone();
    textarea.add_event_listener("input", move || {
      let app = local.lock().unwrap();
      let result = app.on_input();
      app.stderr.update(result);
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

    let local = app.clone();
    button.add_event_listener("click", move || {
      let mut app = local.lock().unwrap();
      let result = app.on_run();
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

  pub(super) fn on_run(&mut self) -> Result {
    self
      .worker
      .post_message(&JsValue::from_str(&serde_json::to_string(
        &AppMessage::Script(&self.textarea.value()),
      )?))
      .map_err(JsValueError)?;
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

  fn on_animation_frame(&mut self, _timestamp: f64) -> Result {
    self.request_animation_frame()?;

    self.gpu.resize()?;

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
      WorkerMessage::Checkbox(name) => {
        let id = Self::widget_id("checkbox", &name);

        if self.document.select_optional(&format!("#{id}"))?.is_none() {
          let aside = self.document.select("aside")?;

          let div = self
            .document
            .create_element("div")
            .map_err(JsValueError)?
            .cast::<HtmlDivElement>()?;

          aside.append_child(&div).map_err(JsValueError)?;

          let label = self
            .document
            .create_element("label")
            .map_err(JsValueError)?
            .cast::<HtmlLabelElement>()?;

          label.set_html_for(&id);
          label.set_inner_text(&name);

          div.append_child(&label).map_err(JsValueError)?;

          let checkbox = self
            .document
            .create_element("input")
            .map_err(JsValueError)?
            .cast::<HtmlInputElement>()?;

          checkbox.set_type("checkbox");
          checkbox.set_id(&id);

          div.append_child(&checkbox).map_err(JsValueError)?;

          let local = checkbox.clone();
          let worker = self.worker.clone();
          let stderr = self.stderr.clone();
          checkbox.add_event_listener("input", move || {
            stderr.update(|| -> Result {
              worker
                .post_message(&JsValue::from_str(&serde_json::to_string(
                  &AppMessage::Checkbox {
                    name: &name,
                    value: local.checked(),
                  },
                )?))
                .map_err(JsValueError)?;
              Ok(())
            }())
          })?;
        }
      }
      WorkerMessage::Clear => {
        self.gpu.clear()?;
      }
      WorkerMessage::DecibelRange { min, max } => {
        self.gpu.set_decibel_range(min, max);
      }
      WorkerMessage::Done => {
        self.html.set_class_name("done");
      }
      WorkerMessage::Error(error) => {
        self.stderr.update(Err(error.into()));
      }
      WorkerMessage::OscillatorFrequency(frequency) => {
        self.oscillator_node.frequency().set_value(frequency);
      }
      WorkerMessage::OscillatorGain(gain) => {
        self.oscillator_gain_node.gain().set_value(gain);
      }
      WorkerMessage::Radio(name, options) => {
        let id = Self::widget_id("radio", &name);

        if self.document.select_optional(&format!("#{id}"))?.is_none() {
          let aside = self.document.select("aside")?;

          let div = self
            .document
            .create_element("div")
            .map_err(JsValueError)?
            .cast::<HtmlDivElement>()?;

          div.set_id(&id);

          aside.append_child(&div).map_err(JsValueError)?;

          let label = self
            .document
            .create_element("label")
            .map_err(JsValueError)?
            .cast::<HtmlLabelElement>()?;

          label.set_html_for(&id);
          label.set_inner_text(&format!("{} ", name));

          div.append_child(&label).map_err(JsValueError)?;

          for (i, option) in options.iter().enumerate() {
            let label = self
              .document
              .create_element("label")
              .map_err(JsValueError)?
              .cast::<HtmlLabelElement>()?;

            label.set_html_for(option);
            label.set_inner_text(option);

            let radio = self
              .document
              .create_element("input")
              .map_err(JsValueError)?
              .cast::<HtmlInputElement>()?;

            radio.set_id(&format!("{}-{}", id, option));
            radio.set_name(&id);
            radio.set_type("radio");

            div.append_child(&label).map_err(JsValueError)?;
            div.append_child(&radio).map_err(JsValueError)?;

            let name = name.clone();
            let option = option.clone();
            let worker = self.worker.clone();
            let stderr = self.stderr.clone();
            radio.add_event_listener("input", move || {
              stderr.update(|| -> Result {
                worker
                  .post_message(&JsValue::from_str(&serde_json::to_string(
                    &AppMessage::Radio {
                      name: &name,
                      value: &option,
                    },
                  )?))
                  .map_err(JsValueError)?;
                Ok(())
              }())
            })?;

            if i == 0 {
              radio.set_checked(true);
            }
          }
        }
      }
      WorkerMessage::Record => {
        if !self.recording {
          let local = self.this();
          let closure = Closure::wrap(Box::new(move |stream: JsValue| {
            let mut app = local.lock().unwrap();
            let result = app.on_get_user_media(stream);
            app.stderr.update(result);
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
        }
      }
      WorkerMessage::Render(filter) => {
        self.gpu.render(&filter)?;
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
      WorkerMessage::Slider {
        name,
        min,
        max,
        initial,
        step,
      } => {
        let id = Self::widget_id("slider", &name);

        if self.document.select_optional(&format!("#{id}"))?.is_none() {
          let aside = self.document.select("aside")?;

          let label = self
            .document
            .create_element("label")
            .map_err(JsValueError)?
            .cast::<HtmlLabelElement>()?;

          label.set_id(&id);
          label.set_inner_text(&name);

          aside.append_child(&label).map_err(JsValueError)?;

          let range = self
            .document
            .create_element("input")
            .map_err(JsValueError)?
            .cast::<HtmlInputElement>()?;

          range.set_type("range");
          range.set_min(&min.to_string());
          range.set_max(&max.to_string());
          range.set_value(&initial.to_string());
          range.set_step(&step.to_string());

          label.append_child(&range).map_err(JsValueError)?;

          let current = self
            .document
            .create_element("span")
            .map_err(JsValueError)?
            .cast::<HtmlSpanElement>()?;
          label.append_child(&current).map_err(JsValueError)?;
          current.set_inner_text(&initial.to_string());

          let local = range.clone();
          let worker = self.worker.clone();
          let stderr = self.stderr.clone();
          range.add_event_listener("input", move || {
            stderr.update(|| -> Result {
              let value = local.value();
              current.set_inner_text(&value);
              worker
                .post_message(&JsValue::from_str(&serde_json::to_string(
                  &AppMessage::Slider {
                    name: &name,
                    value: value.parse()?,
                  },
                )?))
                .map_err(JsValueError)?;
              Ok(())
            }())
          })?;
        }
      }
    }

    Ok(())
  }

  fn on_get_user_media(&mut self, media_stream: JsValue) -> Result {
    let media_stream = media_stream.cast::<MediaStream>()?;

    let media_stream_audio_source_node = self
      .audio_context
      .create_media_stream_source(&media_stream)
      .map_err(JsValueError)?;

    media_stream_audio_source_node
      .connect_with_audio_node(&self.analyser_node)
      .map_err(JsValueError)?;

    self.recording = true;

    Ok(())
  }

  fn on_selection_changed(&mut self) -> Result {
    self.on_input()?;

    self.textarea.set_value(&format!(
      "{}\n// Press the `Run` button or `Shift + Enter` to execute",
      EXAMPLES
        .get(&*self.select.value())
        .ok_or("Failed to get example")?
    ));

    self.textarea.focus().map_err(JsValueError)?;

    Ok(())
  }

  fn on_input(&self) -> Result {
    self
      .html
      .class_list()
      .remove_1("done")
      .map_err(JsValueError)?;

    self
      .nav
      .class_list()
      .add_1("fade-out")
      .map_err(JsValueError)?;

    self.button.set_disabled(false);

    let _promise: Promise = self.audio_context.resume().map_err(JsValueError)?;

    Ok(())
  }

  fn this(&self) -> Arc<Mutex<Self>> {
    self.this.as_ref().unwrap().clone()
  }

  fn widget_id(kind: &str, name: &str) -> String {
    format!("widget-{}-{}", kind, name.replace(' ', "-"))
  }
}
