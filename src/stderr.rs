use super::*;

#[derive(Clone)]
pub(crate) struct Stderr(Option<HtmlElement>);

impl Stderr {
  pub(crate) fn get() -> Self {
    Self(
      window()
        .get_document()
        .select::<HtmlElement>("samp")
        .ok(),
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

    if let Some(stderr_element) = &self.0 {
      stderr_element.prepend_with_node_1(&div)?;
    }

    Ok(())
  }
}
