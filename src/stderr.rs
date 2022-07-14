use super::*;

#[derive(Clone)]
pub(crate) struct Stderr(HtmlElement);

impl Stderr {
  pub(crate) fn get() -> Self {
    Self(
      window()
        .get_document()
        .select::<HtmlElement>("samp")
        .unwrap(),
    )
  }

  pub(crate) fn update(&self, result: Result) {
    if let Err(err) = result {
      self.add(&err).unwrap();
    }
  }

  pub(crate) fn add(&self, err: &dyn std::error::Error) -> Result {
    let message = err.to_string();

    log::error!("{}", message);

    let div = window()
      .get_document()
      .create_element("div")?
      .cast::<HtmlDivElement>()?;

    div.set_inner_text(&message);

    self.0.prepend_with_node_1(&div)?;

    Ok(())
  }
}
