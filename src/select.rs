use super::*;

pub(crate) trait Select {
  fn select_optional<T: JsCast>(&self, selector: &str) -> Result<Option<T>>;

  fn select<T: JsCast>(&self, selector: &str) -> Result<T>;
}

impl Select for Document {
  fn select_optional<T: JsCast>(&self, selector: &str) -> Result<Option<T>> {
    match self.query_selector(selector)? {
      Some(element) => Ok(Some(element.cast::<T>()?)),
      None => Ok(None),
    }
  }

  fn select<T: JsCast>(&self, selector: &str) -> Result<T> {
    Ok(
      self
        .select_optional::<T>(selector)?
        .ok_or_else(|| format!("selector `{}` returned no elements", selector))?,
    )
  }
}
