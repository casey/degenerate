use super::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum WorkerEvent {
  Apply(State),
  Done,
}
