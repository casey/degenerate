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

    Filter::Resize { rows: 20, cols: 80 }.apply(&mut state);

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
