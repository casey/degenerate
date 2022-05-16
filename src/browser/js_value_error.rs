use super::*;

#[derive(Debug)]
pub(crate) struct JsValueError(pub(crate) JsValue);

impl fmt::Display for JsValueError {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "error: {}", self)
  }
}

impl std::error::Error for JsValueError {}
