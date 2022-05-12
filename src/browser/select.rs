use super::*;

pub(crate) trait Select {
  fn select(&self, selector: &str) -> Result<Element>;
}

impl Select for Document {
  fn select(&self, selector: &str) -> Result<Element> {
    Ok(
      self
        .query_selector(selector)
        .map_err(|err| format!("`select` failed: {:?}", err))?
        .ok_or_else(|| format!("selector `{}` returned no elements", selector))?,
    )
  }
}
