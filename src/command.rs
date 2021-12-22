use super::*;

pub(crate) enum Command {
  Filter(Filter),
  Operation(Operation),
  Load { path: PathBuf },
  Repl,
  Resize { rows: usize, cols: usize },
  Save { path: PathBuf },
}

impl Command {
  pub(crate) fn apply(&self, state: &mut State) -> Result<()> {
    match self {
      Self::Filter(filter) => {
        for col in 0..state.matrix.ncols() {
          for row in 0..state.matrix.nrows() {
            if filter.filter(state, col, row) {
              state.matrix[(row, col)] = state.operation.apply(state, state.matrix[(row, col)]);
            }
          }
        }
      }
      Self::Operation(operation) => state.operation = *operation,
      Self::Load { path } => state.load(path)?,
      Self::Repl => {
        for result in BufReader::new(io::stdin()).lines() {
          let line = result?;
          match line.trim().parse::<Command>() {
            Ok(command) => {
              command.apply(state)?;
              state.write(io::stderr())?;
            }
            Err(err) => {
              eprintln!("Could not parse command from `{}`: {}", line, err);
            }
          }
        }
      }
      Self::Resize { rows, cols } => {
        state.resize(Vector2::new(*rows, *cols));
      }
      Self::Save { path } => state.image()?.save(path)?,
    }

    Ok(())
  }
}

impl FromStr for Command {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.split(':').collect::<Vec<&str>>().as_slice() {
      ["all"] => Ok(Self::Filter(Filter::All)),
      ["circle"] => Ok(Self::Filter(Filter::Circle)),
      ["even"] => Ok(Self::Filter(Filter::Even)),
      ["invert"] => Ok(Self::Operation(Operation::Invert)),
      ["load", path] => Ok(Self::Load {
        path: path.parse()?,
      }),
      ["mod", divisor, remainder] => Ok(Self::Filter(Filter::Mod {
        divisor: divisor.parse()?,
        remainder: remainder.parse()?,
      })),
      ["random"] => Ok(Self::Operation(Operation::Random)),
      ["repl"] => Ok(Self::Repl),
      ["resize", cols, rows] => Ok(Self::Resize {
        cols: cols.parse()?,
        rows: rows.parse()?,
      }),
      ["save", path] => Ok(Self::Save {
        path: path.parse()?,
      }),
      ["square"] => Ok(Self::Filter(Filter::Square)),
      ["top"] => Ok(Self::Filter(Filter::Top)),
      _ => Err(format!("Invalid command: {}", s).into()),
    }
  }
}
