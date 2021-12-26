use super::*;

#[derive(Clone, Debug)]
pub(crate) enum Command {
  Filter(Filter),
  For(usize),
  Load(Option<PathBuf>),
  Loop,
  Operation(Operation),
  Print,
  Repl,
  Resize((usize, usize)),
  Save(Option<PathBuf>),
  Verbose,
}

impl Command {
  pub(crate) fn apply(&self, state: &mut State) -> Result<()> {
    match self {
      Self::Filter(filter) => {
        for col in 0..state.matrix.ncols() {
          for row in 0..state.matrix.nrows() {
            let pixel = Vector2::new(col, row);
            if filter.filter(state, pixel, pixel.coordinates(state.dimensions())) {
              state.matrix[(row, col)] = state.operation.apply(state, state.matrix[(row, col)]);
            }
          }
        }
      }
      Self::For(until) => {
        if state.loop_counter >= *until {
          loop {
            state.program_counter = state.program_counter.wrapping_add(1);
            if let Some(Self::Loop) | None = state.program.get(state.program_counter) {
              break;
            }
          }
          state.loop_counter = 0;
        }
      }
      Self::Load(path) => state.load(path.as_deref().unwrap_or(Path::new("output.png")))?,
      Self::Loop => {
        loop {
          state.program_counter = state.program_counter.wrapping_sub(1);
          let next = state.program_counter.wrapping_add(1);
          if next == 0 {
            break;
          }
          if let Some(Self::For(_)) | None = state.program.get(next) {
            break;
          }
        }
        state.loop_counter += 1;
      }
      Self::Operation(operation) => state.operation = *operation,
      Self::Print => state.print()?,
      Self::Repl => {
        let history = home_dir().unwrap_or_default().join(".degenerate_history");

        let mut editor = Editor::<()>::new();
        editor.load_history(&history).ok();

        loop {
          let line = editor.readline("> ")?;

          editor.add_history_entry(line.as_str());
          editor.save_history(&history)?;

          match line.parse::<Self>() {
            Ok(command) => {
              command.apply(state)?;
              state.print()?;
            }
            Err(err) => {
              eprintln!("Could not parse command from `{}`: {}", line, err);
            }
          }
        }
      }
      Self::Resize(dimensions) => {
        state.resize(*dimensions);
      }
      Self::Save(path) => state
        .image()?
        .save(path.as_deref().unwrap_or(Path::new("output.png")))?,
      Self::Verbose => state.verbose = !state.verbose,
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
      ["cross"] => Ok(Self::Filter(Filter::Cross)),
      ["for", count] => Ok(Self::For(count.parse()?)),
      ["invert"] => Ok(Self::Operation(Operation::Invert)),
      ["load"] => Ok(Self::Load(None)),
      ["load", path] => Ok(Self::Load(Some(path.parse()?))),
      ["loop"] => Ok(Self::Loop),
      ["mod", divisor, remainder] => Ok(Self::Filter(Filter::Mod {
        divisor: divisor.parse()?,
        remainder: remainder.parse()?,
      })),
      ["rows", nrows, step] => Ok(Self::Filter(Filter::Rows {
        nrows: nrows.parse()?,
        step: step.parse()?,
      })),
      ["print"] => Ok(Self::Print),
      ["random"] => Ok(Self::Operation(Operation::Random)),
      ["repl"] => Ok(Self::Repl),
      ["resize", cols, rows] => Ok(Self::Resize((rows.parse()?, cols.parse()?))),
      ["save"] => Ok(Self::Save(None)),
      ["save", path] => Ok(Self::Save(Some(path.parse()?))),
      ["top"] => Ok(Self::Filter(Filter::Top)),
      ["verbose"] => Ok(Self::Verbose),
      ["square"] => Ok(Self::Filter(Filter::Square)),
      ["x"] => Ok(Self::Filter(Filter::X)),
      _ => Err(format!("Invalid command: {}", s).into()),
    }
  }
}
