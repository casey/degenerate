use super::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum WorkerMessage {
  Render(State),
  Done,
}
