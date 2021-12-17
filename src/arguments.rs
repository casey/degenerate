use super::*;

#[derive(StructOpt)]
pub(crate) struct Arguments {
  #[structopt(long)]
  pub(crate) text_bitmap: bool,
  pub(crate) filters: Vec<Filter>,
  #[structopt(long)]
  pub(crate) output: Option<PathBuf>,
}
