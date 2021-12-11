use super::*;

#[derive(StructOpt)]
pub(crate) struct Arguments {
  pub(crate) filters: Vec<Filter>,
}
