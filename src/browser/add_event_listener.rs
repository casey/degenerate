use super::*;

pub(crate) trait AddEventListener {
  fn add_event_listener(&self, event: &str, function: impl FnMut() + 'static) -> Result;
}

impl<T: Deref<Target = EventTarget>> AddEventListener for T {
  fn add_event_listener(&self, event: &str, function: impl FnMut() + 'static) -> Result {
    let closure = Closure::wrap(Box::new(function) as Box<dyn FnMut()>);
    self
      .deref()
      .add_event_listener_with_callback(event, &closure.as_ref().dyn_ref().unwrap())
      .map_err(|err| format!("Failed to set event listener: {:?}", err))?;
    closure.forget();
    Ok(())
  }
}
