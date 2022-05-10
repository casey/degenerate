use super::*;

pub(crate) trait GetDocument {
  fn get_document(&self) -> Document;
}

impl GetDocument for Window {
  fn get_document(&self) -> Document {
    self.document().expect("`window.document` missing")
  }
}
