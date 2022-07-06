use super::*;

pub(crate) struct App {
  animation_frame_callback: Option<Closure<dyn FnMut(f64)>>,
  button: HtmlButtonElement,
  document: Document,
  gpu: Gpu,
  html: HtmlElement,
  nav: HtmlElement,
  select: HtmlSelectElement,
  stderr: Stderr,
  textarea: HtmlTextAreaElement,
  window: Window,
  worker: Worker,
}

const EXAMPLES: include_dir::Dir = include_dir!("examples");

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

    for entry in EXAMPLES.entries() {
      match entry {
        include_dir::DirEntry::File(file) => {
          let path = file.path();

          let option = document
            .create_element("option")
            .map_err(JsValueError)?
            .cast::<HtmlOptionElement>()?;

          option.set_text(
            &path
              .file_stem()
              .ok_or("Failed to extract file stem")?
              .to_str()
              .ok_or("Failed to convert OsStr to str")?
              .split('_')
              .into_iter()
              .map(|s| s[0..1].to_uppercase() + &s[1..])
              .collect::<Vec<String>>()
              .join(" "),
          );

          option.set_value(path.to_str().ok_or("Failed to convert path to str")?);

          select
            .add_with_html_option_element(&option)
            .map_err(JsValueError)?
        }
        _ => continue,
      }
    }

    let stderr = Stderr::get();

    let gpu = Gpu::new(&canvas, &window)?;

    let worker = Worker::new("/worker.js").map_err(JsValueError)?;

    let app = Arc::new(Mutex::new(Self {
      animation_frame_callback: None,
      button: button.clone(),
      document,
      gpu,
      html,
      nav,
      select: select.clone(),
      stderr,
      textarea: textarea.clone(),
      window,
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
      WorkerMessage::Checkbox(name) => {
        let id = format!("widget-checkbox-{name}");

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
      WorkerMessage::Done => {
        self.html.set_class_name("done");
      }
      WorkerMessage::Error(error) => {
        self.stderr.update(Err(error)?);
      }
      WorkerMessage::Radio(name, options) => {
        let id = format!("widget-radio-{name}");

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

            radio.set_id(option);
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

  fn on_selection_changed(&mut self) -> Result {
    self.on_input()?;

    self.textarea.set_value(&format!(
      "{}\n// Press the `Run` button or `Shift + Enter` to execute",
      EXAMPLES
        .get_file(Path::new(&self.select.value()))
        .ok_or("Failed to get file")?
        .contents_utf8()
        .ok_or("Failed to get file contents")?
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

    Ok(())
  }
}
