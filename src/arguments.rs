use super::*;

#[derive(StructOpt)]
pub(crate) struct Arguments {
  pub(crate) filters: Vec<Filter>,
  #[structopt(long)]
  pub(crate) output: Option<PathBuf>,
}

impl Arguments {
  pub(crate) fn run(self) -> Result<()> {
    let mut state = State::new();

    for filter in self.filters {
      filter.apply(&mut state);
    }

    if let Some(path) = self.output {
      state.save(path)?;
    } else {
      state.write()?;
    }

    Ok(())
  }
}
