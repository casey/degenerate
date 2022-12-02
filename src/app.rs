use super::*;

pub(crate) struct App {
  analyser_node: AnalyserNode,
  animation_frame_callback: Option<Closure<dyn FnMut(f64)>>,
  aside: HtmlElement,
  audio_context: AudioContext,
  document: Document,
  gpu: Gpu,
  html: HtmlElement,
  nav: HtmlElement,
  oscillator_gain_node: GainNode,
  oscillator_node: OscillatorNode,
  recording: bool,
  run_button: HtmlButtonElement,
  select: HtmlSelectElement,
  share_button: HtmlButtonElement,
  started: bool,
  stderr: Stderr,
  textarea: HtmlTextAreaElement,
  this: Option<Arc<Mutex<Self>>>,
  window: Window,
  worker: Worker,
}

lazy_static! {
  static ref EXAMPLES: BTreeMap<&'static str, &'static str> = [
    ("All", include_str!("../examples/all.js")),
    ("Kaleidoscope", include_str!("../examples/kaleidoscope.js")),
    ("Orb Zoom", include_str!("../examples/orb_zoom.js")),
    ("Orbs", include_str!("../examples/orbs.js")),
    ("Pattern", include_str!("../examples/pattern.js")),
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

    let body = document.body().ok_or("failed to get document body")?;

    let html = document.select::<HtmlElement>("html")?;

    let main = document.select::<HtmlElement>("main")?;

    let textarea = document.select::<HtmlTextAreaElement>("textarea")?;

    let canvas = document.select::<HtmlCanvasElement>("canvas")?;

    let nav = document.select::<HtmlElement>("nav")?;

    let select = document.select::<HtmlSelectElement>("select")?;

    let run_button = document.select::<HtmlButtonElement>("button#run")?;

    let share_button = document.select::<HtmlButtonElement>("button#share")?;

    for name in EXAMPLES.keys() {
      let option = document
        .create_element("option")?
        .cast::<HtmlOptionElement>()?;

      option.set_text(name);

      option.set_value(name);

      select.add_with_html_option_element(&option)?;
    }

    let stderr = Stderr::get();

    let audio_context = AudioContext::new()?;

    let analyser_node = audio_context.create_analyser()?;

    analyser_node.set_smoothing_time_constant(0.1);

    let gpu = Gpu::new(&window, &canvas, &analyser_node)?;

    let location = window.location();

    let loader = location.pathname()? == "/loader";

    let worker = if loader {
      let mut worker_options = WorkerOptions::new();
      worker_options.type_(WorkerType::Module);
      Worker::new_with_options("/loader.js", &worker_options)?
    } else {
      main.class_list().add_1("fade-in")?;
      Worker::new("/interpreter.js")?
    };

    let oscillator_gain_node = audio_context.create_gain()?;
    oscillator_gain_node.gain().set_value(0.0);

    let oscillator_node = audio_context.create_oscillator()?;
    oscillator_node.frequency().set_value(60.0);

    oscillator_node.connect_with_audio_node(&oscillator_gain_node)?;
    oscillator_node.start()?;

    oscillator_gain_node.connect_with_audio_node(&audio_context.destination())?;

    oscillator_gain_node.connect_with_audio_node(&analyser_node)?;

    let app = Arc::new(Mutex::new(Self {
      analyser_node,
      animation_frame_callback: None,
      aside: document.select::<HtmlElement>("aside")?,
      audio_context,
      document,
      gpu,
      html,
      nav,
      oscillator_gain_node,
      oscillator_node,
      recording: false,
      run_button: run_button.clone(),
      select: select.clone(),
      share_button: share_button.clone(),
      started: false,
      stderr: stderr.clone(),
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

    if loader {
      Self::add_event_listener(&app, &body, "click", move |app| app.on_click())?;
    }

    Self::add_event_listener(&app, &textarea, "input", move |app| app.on_input())?;

    Self::add_event_listener_with_event(&app, &textarea, "keydown", move |app, event| {
      app.on_keydown(event)
    })?;

    Self::add_event_listener(&app, &select, "change", move |app| {
      app.on_selection_changed()
    })?;

    Self::add_event_listener_with_event(&app, &worker, "message", move |app, event| {
      app.on_message(event)
    })?;

    Self::add_event_listener(&app, &run_button, "click", move |app| app.on_run())?;

    Self::add_event_listener(&app, &share_button, "click", move |app| app.on_share())?;

    let path = location.pathname()?;

    match path.split_inclusive('/').collect::<Vec<&str>>().as_slice() {
      ["/"] | ["/", "loader"] => {}
      ["/", "program/" | "program"] => {
        location.set_pathname("/")?;
      }
      ["/", "program/", hex] => {
        let bytes = hex::decode(hex)?;
        let script = str::from_utf8(&bytes)?;
        textarea.set_value(script);
        app.lock().unwrap().run_script(script)?;
      }
      _ => stderr.update(Err(format!("Unrecognized path: {}", path).into())),
    }

    let mut app = app.lock().unwrap();
    app.request_animation_frame()?;
    app.html.set_class_name("ready");

    Ok(())
  }

  fn add_event_listener(
    app: &Arc<Mutex<Self>>,
    target: &EventTarget,
    name: &str,
    callback: impl Fn(&mut Self) -> Result + 'static,
  ) -> Result {
    Self::add_event_listener_with_event(app, target, name, move |app, _: web_sys::Event| {
      callback(app)
    })
  }

  fn add_event_listener_with_event<E: FromWasmAbi + 'static>(
    app: &Arc<Mutex<Self>>,
    target: &EventTarget,
    name: &str,
    callback: impl Fn(&mut Self, E) -> Result + 'static,
  ) -> Result {
    let local = app.clone();
    target.add_event_listener_with_event(name, move |event| {
      let mut app = local.lock().unwrap();
      let result = callback(&mut app, event);
      app.stderr.update(result);
    })
  }

  pub(super) fn on_keydown(&mut self, event: KeyboardEvent) -> Result {
    if event.shift_key() && event.key() == "Enter" {
      event.prevent_default();
      self.run_script(&self.textarea.value())?;
    }

    if event.key() == " " {
      self.on_input()?;
      self
        .worker
        .post_message(&JsValue::from_str(&serde_json::to_string(&Event::Beat)?))?;
    }

    Ok(())
  }

  pub(super) fn on_run(&mut self) -> Result {
    self.run_script(&self.textarea.value())
  }

  fn request_animation_frame(&mut self) -> Result {
    self.window.request_animation_frame(
      self
        .animation_frame_callback
        .as_ref()
        .unwrap()
        .as_ref()
        .dyn_ref()
        .unwrap(),
    )?;
    Ok(())
  }

  pub(super) fn run_script(&self, script: &str) -> Result {
    self
      .worker
      .post_message(&JsValue::from_str(&serde_json::to_string(&Event::Script(
        script.into(),
      ))?))?;
    Ok(())
  }

  fn on_animation_frame(&mut self, timestamp: f64) -> Result {
    self.request_animation_frame()?;

    self.gpu.resize()?;

    self
      .worker
      .post_message(&JsValue::from_str(&serde_json::to_string(&Event::Frame(
        timestamp as f32,
      ))?))?;

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
      Message::Clear => {
        self.gpu.clear()?;
      }
      Message::DecibelRange { min, max } => {
        self.gpu.set_decibel_range(min, max);
      }
      Message::Done => {
        self.html.set_class_name("done");
      }
      Message::Error(error) => {
        self.stderr.update(Err(error.into()));
      }
      Message::OscillatorFrequency(frequency) => {
        self.oscillator_node.frequency().set_value(frequency);
      }
      Message::OscillatorGain(gain) => {
        self.oscillator_gain_node.gain().set_value(gain);
      }
      Message::Record => {
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
            .media_devices()?
            .get_user_media_with_constraints(
              MediaStreamConstraints::new()
                .audio(&true.into())
                .video(&false.into()),
            )?
            .then(&closure);
          closure.forget();
        }
      }
      Message::Render(filter) => {
        self.gpu.render(&filter)?;
        self.gpu.present()?;
      }
      Message::Resolution(resolution) => {
        self.gpu.lock_resolution(resolution);
      }
      Message::Save => {
        let image = self.gpu.save_image()?;
        let mut png = Cursor::new(Vec::new());
        image.write_to(&mut png, ImageOutputFormat::Png)?;
        let a = self
          .document
          .create_element("a")?
          .cast::<HtmlAnchorElement>()?;
        a.set_download("degenerate.png");
        let mut href = String::from("data:image/png;base64,");
        base64::encode_config_buf(png.get_ref(), base64::STANDARD, &mut href);
        a.set_href(&href);
        a.click();
      }
      Message::Widget { name, widget } => {
        let id = widget.id(&name);

        if self.document.get_element_by_id(&id).is_none() {
          let key = widget.key(&name);

          let label = self
            .document
            .create_element("label")?
            .cast::<HtmlLabelElement>()?;

          label.set_id(&id);
          label.set_inner_text(&name);

          self.aside.append_child(&label)?;

          match widget {
            Widget::Checkbox => {
              let checkbox = self
                .document
                .create_element("input")?
                .cast::<HtmlInputElement>()?;

              checkbox.set_type("checkbox");

              label.append_child(&checkbox)?;

              let local = checkbox.clone();
              let worker = self.worker.clone();
              let stderr = self.stderr.clone();
              checkbox.add_event_listener("input", move || {
                stderr.update(|| -> Result {
                  worker.post_message(&JsValue::from_str(&serde_json::to_string(
                    &Event::Widget {
                      key: key.clone(),
                      value: serde_json::Value::Bool(local.checked()),
                    },
                  )?))?;
                  Ok(())
                }())
              })?;
            }
            Widget::Radio { options } => {
              for (i, option) in options.iter().enumerate() {
                let option_label = self
                  .document
                  .create_element("label")?
                  .cast::<HtmlLabelElement>()?;

                option_label.set_html_for(option);
                option_label.set_inner_text(option);
                label.append_child(&option_label)?;

                let radio = self
                  .document
                  .create_element("input")?
                  .cast::<HtmlInputElement>()?;

                radio.set_id(&format!("{}-{}", id, option));
                radio.set_name(&id);
                radio.set_type("radio");
                option_label.append_child(&radio)?;

                let option = option.clone();
                let worker = self.worker.clone();
                let key = key.clone();
                let stderr = self.stderr.clone();
                radio.add_event_listener("input", move || {
                  stderr.update(|| -> Result {
                    worker.post_message(&JsValue::from_str(&serde_json::to_string(
                      &Event::Widget {
                        key: key.clone(),
                        value: serde_json::Value::String(option.clone()),
                      },
                    )?))?;
                    Ok(())
                  }())
                })?;

                if i == 0 {
                  radio.set_checked(true);
                }
              }
            }
            Widget::Slider {
              min,
              max,
              step,
              initial,
            } => {
              let range = self
                .document
                .create_element("input")?
                .cast::<HtmlInputElement>()?;

              range.set_type("range");
              range.set_min(&min.to_string());
              range.set_max(&max.to_string());
              range.set_step(&step.to_string());
              range.set_default_value(&initial.to_string());

              label.append_child(&range)?;

              let current = self
                .document
                .create_element("span")?
                .cast::<HtmlSpanElement>()?;
              label.append_child(&current)?;
              current.set_inner_text(&initial.to_string());

              let local = range.clone();
              let worker = self.worker.clone();
              let stderr = self.stderr.clone();
              range.add_event_listener("input", move || {
                stderr.update(|| -> Result {
                  let value = local.value();
                  current.set_inner_text(&value);
                  worker.post_message(&JsValue::from_str(&serde_json::to_string(
                    &Event::Widget {
                      key: key.clone(),
                      value: serde_json::Value::Number(value.parse()?),
                    },
                  )?))?;
                  Ok(())
                }())
              })?;
            }
          }
        }
      }
    }

    Ok(())
  }

  fn on_get_user_media(&mut self, media_stream: JsValue) -> Result {
    let media_stream = media_stream.cast::<MediaStream>()?;

    let media_stream_audio_source_node = self
      .audio_context
      .create_media_stream_source(&media_stream)?;

    media_stream_audio_source_node.connect_with_audio_node(&self.analyser_node)?;

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

    self.textarea.focus()?;

    Ok(())
  }

  pub(super) fn on_share(&mut self) -> Result {
    let script = self.textarea.value();

    if !script.is_empty() {
      let hex = hex::encode(script);
      let path = format!("/program/{hex}");
      self
        .window
        .history()?
        .replace_state_with_url(&JsValue::NULL, "", Some(&path))?;
    }

    Ok(())
  }

  fn start(&mut self) -> Result {
    if !self.started {
      self.html.class_list().remove_1("done")?;
      self.nav.class_list().add_1("fade-out")?;
      self.run_button.set_disabled(false);
      self.share_button.set_disabled(false);
      let _: Promise = self.audio_context.resume()?;
      self.started = true;
    }
    Ok(())
  }

  fn on_click(&mut self) -> Result {
    self.start()?;
    self
      .worker
      .post_message(&JsValue::from_str(&serde_json::to_string(&Event::Beat)?))?;
    Ok(())
  }

  fn on_input(&mut self) -> Result {
    self.start()?;
    Ok(())
  }

  fn this(&self) -> Arc<Mutex<Self>> {
    self.this.as_ref().unwrap().clone()
  }
}
