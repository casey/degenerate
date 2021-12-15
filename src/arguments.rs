use super::*;

#[derive(StructOpt)]
pub(crate) struct Arguments {
  pub(crate) filters: Vec<Filter>,
  #[structopt(long)]
  pub(crate) output: Option<PathBuf>,
}
