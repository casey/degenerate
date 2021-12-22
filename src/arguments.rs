use super::*;

#[derive(StructOpt)]
pub(crate) struct Arguments {
  pub(crate) commands: Vec<Command>,
  #[structopt(long)]
  pub(crate) output: Option<PathBuf>,
}

impl Arguments {
  pub(crate) fn run(self) -> Result<()> {
    let mut state = State::new();

    if self.output.is_some() {
      Command::Resize {
        rows: 4096,
        cols: 4096,
      }
      .apply(&mut state)?;
    } else {
      Command::Resize { rows: 20, cols: 80 }.apply(&mut state)?;
    }

    for command in self.commands {
      command.apply(&mut state)?;
    }

    if let Some(path) = self.output {
      state.save(path)?;
    } else {
      state.write(io::stdout())?;
    }

    Ok(())
  }
}
