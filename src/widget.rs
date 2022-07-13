use super::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Widget {
  Checkbox,
  Slider {
    initial: f64,
    max: f64,
    min: f64,
    step: f64,
  },
  Radio {
    options: Vec<String>,
  },
}

impl Widget {
  pub(crate) fn id(&self, name: &str) -> String {
    format!("widget-{}-{name}", self.kind())
  }

  pub(crate) fn key(&self, name: &str) -> String {
    format!("{}-{name}", self.kind())
  }

  fn kind(&self) -> &str {
    match self {
      Self::Checkbox => "checkbox",
      Self::Slider { .. } => "slider",
      Self::Radio { .. } => "radio",
    }
  }
}
