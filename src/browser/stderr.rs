use super::*;

pub(crate) struct Stderr(Node);

impl Stderr {
  pub(crate) fn get() -> Result<Self> {
    Ok(Self(
      window().get_document().select("samp")?.cast::<Node>()?,
    ))
  }

  pub(crate) fn set(&self, err: &dyn std::error::Error) {
    self.0.set_text_content(Some(&err.to_string()));
  }

  pub(crate) fn clear(&self) {
    self.0.set_text_content(None);
  }
}
