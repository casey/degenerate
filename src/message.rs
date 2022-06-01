use super::*;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum MessageType {
  Script,
  Run,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Message<'a> {
  pub(crate) message_type: MessageType,
  pub(crate) payload: Option<&'a str>,
}
