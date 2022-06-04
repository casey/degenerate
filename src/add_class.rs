use super::*;

pub(crate) trait AddClass {
  fn add_class(&self, class: &str) -> Result;
}

impl AddClass for Element {
  fn add_class(&self, class: &str) -> Result {
    Ok(self.class_list().add_1(class).map_err(JsValueError)?)
  }
}
