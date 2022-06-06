use super::*;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "tag", content = "content")]
pub(crate) enum AppMessage<'a> {
  Script(&'a str),
  Frame,
}
