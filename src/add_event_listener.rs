use super::*;

pub(crate) trait AddEventListener {
  fn add_event_listener(&self, event: &str, function: impl FnMut() + 'static) -> Result;
  fn add_event_listener_with_event<E: FromWasmAbi + 'static, F: FnMut(E) + 'static>(
    &self,
    event: &str,
    function: F,
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

  fn add_event_listener_with_event<E: FromWasmAbi + 'static, F: FnMut(E) + 'static>(
    &self,
    event: &str,
    function: F,
  ) -> Result {
    let closure = Closure::wrap(Box::new(function) as Box<dyn FnMut(E)>);
    self
      .deref()
      .add_event_listener_with_callback(event, closure.as_ref().dyn_ref().unwrap())
      .map_err(JsValueError)?;
    closure.forget();
    Ok(())
  }
}
