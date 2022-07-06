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
      Err(err) => self.add(err.as_ref()).unwrap(),
      _ => {}
    }
  }

  pub(crate) fn add(&self, err: &dyn std::error::Error) -> Result {
    let message = err.to_string();

    log::error!("{}", message);

    let div = window()
      .get_document()
      .create_element("div")
      .map_err(JsValueError)?
      .cast::<HtmlDivElement>()?;

    div.set_class_name("error");
    div.set_inner_text(&message);

    self.0.append_child(&div).map_err(JsValueError)?;

    Ok(())
  }
}
