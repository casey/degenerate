use super::*;

#[derive(Clone)]
pub(crate) struct Stderr(HtmlElement);

impl Stderr {
  pub(crate) fn get() -> Self {
    Self(
      window()
        .get_document()
        .select("samp")
        .unwrap()
        .cast::<HtmlElement>()
        .unwrap(),
    )
  }

  pub(crate) fn update(&self, result: Result) {
    match result {
      Err(err) => self.set(err.as_ref()),
      Ok(()) => self.clear(),
    }
  }

  pub(crate) fn set(&self, err: &dyn std::error::Error) {
    let message = err.to_string();
    log::error!("{}", message);
    self.0.set_text_content(Some(&message));
  }

  pub(crate) fn clear(&self) {
    // self.0.set_text_content(None);
  }
}
