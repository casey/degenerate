use super::*;

pub(crate) trait Cast {
  fn cast<T: JsCast>(self) -> Result<T>;
}

impl<V: JsCast + std::fmt::Debug> Cast for V {
  fn cast<T: JsCast>(self) -> Result<T> {
    Ok(
      self
        .dyn_into::<T>()
        .map_err(|value| format!("`cast` failed for value: {:?}", value))?,
    )
  }
}
