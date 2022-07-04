use super::*;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "tag", content = "content")]
pub(crate) enum AppMessage<'a> {
  Checkbox { name: &'a str, value: bool },
  Frame,
  Radio { name: &'a str, value: &'a str },
  Script(&'a str),
}
