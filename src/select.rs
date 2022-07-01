use super::*;

pub(crate) trait Select {
  fn select(&self, selector: &str) -> Result<Element>;

  fn select_optional(&self, selector: &str) -> Result<Option<Element>>;
}

impl Select for Document {
  fn select(&self, selector: &str) -> Result<Element> {
    Ok(
      self
        .select_optional(selector)?
        .ok_or_else(|| format!("selector `{}` returned no elements", selector))?,
    )
  }

  fn select_optional(&self, selector: &str) -> Result<Option<Element>> {
    Ok(self.query_selector(selector).map_err(JsValueError)?)
  }
}
