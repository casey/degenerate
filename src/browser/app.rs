use {super::*, std::ops::Deref, web_sys::EventTarget};

pub(crate) struct App {
  canvas: HtmlCanvasElement,
  nav: HtmlElement,
  resize: bool,
  window: Window,
  textarea: HtmlTextAreaElement,
  display: Display,
}

trait AddEventListenerWithFunction {
  fn add_event_listener_with_function(
    &self,
    event: &str,
    function: impl FnMut() + 'static,
  ) -> Result;
}

impl<T: Deref<Target = EventTarget>> AddEventListenerWithFunction for T {
  fn add_event_listener_with_function(
    &self,
    event: &str,
    function: impl FnMut() + 'static,
  ) -> Result {
    let closure = Closure::wrap(Box::new(function) as Box<dyn FnMut()>);
    self
      .deref()
      .add_event_listener_with_callback(event, &closure.as_ref().dyn_ref().unwrap())
      .map_err(|err| format!("Failed to set event listener: {:?}", err))?;
    closure.forget();
    Ok(())
  }
}

impl App {
  pub(super) fn init() -> Result {
    let window = window();

    let document = window.get_document();

    let textarea = document.select("textarea")?.cast::<HtmlTextAreaElement>()?;

    let canvas = document.select("canvas")?.cast::<HtmlCanvasElement>()?;

    let nav = document.select("nav")?.cast::<HtmlElement>()?;

    let stderr = Stderr::get()?;

    let context = canvas
      .get_context("2d")
      .map_err(|err| format!("`canvas.get_context(\"2d\")` failed: {:?}", err))?
      .ok_or_else(|| format!("failed to retrieve context"))?
      .cast::<CanvasRenderingContext2d>()?;

    let app = Arc::new(Mutex::new(Self {
      window: window.clone(),
      resize: true,
      display: Display { context },
      textarea: textarea.clone(),
      nav,
      canvas,
    }));

    let local = app.clone();
    window.add_event_listener_with_function("resize", move || local.lock().unwrap().on_resize())?;

    let local = app.clone();
    textarea.add_event_listener_with_function("input", move || {
      match local.lock().unwrap().on_input() {
        Err(err) => stderr.set(err.as_ref()),
        Ok(()) => stderr.clear(),
      }
    })?;

    Ok(())
  }

  pub(super) fn on_resize(&mut self) {
    log!("resize");
    self.resize = true;
  }

  pub(super) fn on_input(&mut self) -> Result {
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
    self.nav.set_class_name("fade-out");
    Computer::run(&self.display, self.textarea.value().split_whitespace())
  }
}
