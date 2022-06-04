use super::*;

pub(crate) trait AddEventListener {
  fn add_event_listener(&self, event: &str, function: impl FnMut() + 'static) -> Result;
  fn add_event_listener_with_event(
    &self,
    event: &str,
    function: impl FnMut(MessageEvent) -> Result<(), String> + 'static,
  ) -> Result;
}

impl<T: Deref<Target = EventTarget>> AddEventListener for T {
  fn add_event_listener(&self, event: &str, function: impl FnMut() + 'static) -> Result {
    let closure = Closure::wrap(Box::new(function) as Box<dyn FnMut()>);
    self
      .deref()
      .add_event_listener_with_callback(event, closure.as_ref().dyn_ref().unwrap())
      .map_err(JsValueError)?;
    closure.forget();
    Ok(())
  }

  fn add_event_listener_with_event(
    &self,
    event: &str,
    function: impl FnMut(MessageEvent) -> Result<(), String> + 'static,
  ) -> Result {
    let closure = Closure::wrap(Box::new(function) as Box<dyn FnMut(MessageEvent) -> Result<(), String>>);
    self
      .deref()
      .add_event_listener_with_callback(event, closure.as_ref().dyn_ref().unwrap())
      .map_err(JsValueError)?;
    closure.forget();
    Ok(())
  }
}
