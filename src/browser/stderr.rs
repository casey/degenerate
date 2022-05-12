use super::*;

#[derive(Clone)]
pub(crate) struct Stderr(HtmlElement);

impl Stderr {
  pub(crate) fn get() -> Result<Self> {
    Ok(Self(
      window()
        .get_document()
        .select("samp")?
        .cast::<HtmlElement>()?,
    ))
  }

  pub(crate) fn update(&self, result: Result) {
    match result {
      Err(err) => self.set(err.as_ref()),
      Ok(()) => self.clear(),
    }
  }

  pub(crate) fn set(&self, err: &dyn std::error::Error) {
    self.0.set_text_content(Some(&err.to_string()));
  }

  pub(crate) fn clear(&self) {
    self.0.set_text_content(None);
  }
}
