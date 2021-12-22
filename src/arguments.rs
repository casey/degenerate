use super::*;

#[derive(StructOpt)]
pub(crate) struct Arguments {
  pub(crate) commands: Vec<Command>,
}

impl Arguments {
  pub(crate) fn run(self) -> Result<()> {
    let mut state = State::new();

    for command in self.commands {
      command.apply(&mut state)?;
    }

    Ok(())
  }
}
