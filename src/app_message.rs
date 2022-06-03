use super::*;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum AppMessage<'a> {
  Script(&'a str),
  Run,
}
