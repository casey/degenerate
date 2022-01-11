use super::*;

const DEFAULT_OUTPUT_PATH: &str = "output.png";

#[derive(Clone, Debug)]
pub(crate) enum Command {
  Apply,
  Autosave,
  Comment,
  Default(Vector3<u8>),
  For(usize),
  Generate,
  Load(Option<PathBuf>),
  Loop,
  Mask(Mask),
  Open(Option<PathBuf>),
  Operation(Operation),
  Print,
  RandomMask,
  Repl,
  Resize((usize, usize)),
  Rotate(f64),
  Save(Option<PathBuf>),
  Scale(f64),
  Seed(u64),
  Verbose,
  Wrap,
}

impl Command {
  pub(crate) fn run(&self, state: &mut State) -> Result<()> {
    match self {
      Self::Apply => {
        let similarity = state.similarity.inverse();
        let mut output = state.matrix.clone();
        for col in 0..state.matrix.ncols() {
          for row in 0..state.matrix.nrows() {
            let i = Vector2::new(col, row);
            let v = i.coordinates(state.dimensions());
            let v = similarity * v;
            let v = if state.wrap { v.wrap() } else { v };
            let i = v.pixel(state.dimensions());
            if state.mask.is_masked(state, i, v) {
              output[(row, col)] = state.operation.apply(
                if i.x >= 0
                  && i.y >= 0
                  && i.x < state.matrix.ncols() as isize
                  && i.y < state.matrix.nrows() as isize
                {
                  state.matrix[(i.y as usize, i.x as usize)]
                } else {
                  state.default
                },
              );
            }
          }
        }
        state.matrix = output;
        state.autosave()?;
      }
      Self::Autosave => state.autosave = !state.autosave,
      Self::Comment => {}
      Self::Default(default) => {
        state.default = *default;
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
      Self::Generate => {
        state.program.splice(
          state.program_counter + 1..state.program_counter + 1,
          [
            Command::RandomMask,
            Command::Scale(0.99),
            Command::For(100),
            Command::Apply,
            Command::Loop,
          ],
        );
      }
      Self::Load(path) => state.load(
        path
          .as_deref()
          .unwrap_or_else(|| DEFAULT_OUTPUT_PATH.as_ref()),
      )?,
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
      Self::Open(path) => {
        process::Command::new(
          env::var("DEGENERATE_OPEN_COMMAND").unwrap_or(
            if cfg!(target_os = "macos") {
              "open".to_string()
            } else if cfg!(target_os = "linux") {
              "xdg-open".to_string()
            } else {
              return Err("Please supply an open command by setting the `DEGENERATE_OPEN_COMMAND` environment variable".into())
            }
          )
        )
        .arg(path.as_deref().unwrap_or_else(|| DEFAULT_OUTPUT_PATH.as_ref()))
        .spawn()?;
      }
      Self::Mask(mask) => {
        state.mask = mask.clone();
      }
      Self::Operation(operation) => state.operation = *operation,
      Self::Print => state.print()?,
      Self::RandomMask => Self::Mask(state.rng.gen()).run(state)?,
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
              command.run(state)?;
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
        state.autosave()?;
      }
      Self::Rotate(turns) => state
        .similarity
        .append_rotation_mut(&UnitComplex::from_angle(turns * f64::consts::TAU)),
      Self::Save(path) => state.image()?.save(
        path
          .as_deref()
          .unwrap_or_else(|| DEFAULT_OUTPUT_PATH.as_ref()),
      )?,
      Self::Scale(scaling) => {
        state.similarity.append_scaling_mut(*scaling);
      }
      Self::Seed(seed) => state.rng = StdRng::seed_from_u64(*seed),
      Self::Verbose => state.verbose = !state.verbose,
      Self::Wrap => state.wrap = !state.wrap,
    }

    Ok(())
  }
}

impl FromStr for Command {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.split(':').collect::<Vec<&str>>().as_slice() {
      ["all"] => Ok(Self::Mask(Mask::All)),
      ["apply"] => Ok(Self::Apply),
      ["autosave"] => Ok(Self::Autosave),
      ["circle"] => Ok(Self::Mask(Mask::Circle)),
      ["comment", ..] => Ok(Self::Comment),
      ["cross"] => Ok(Self::Mask(Mask::Cross)),
      ["default", r, g, b] => Ok(Self::Default(Vector3::new(
        r.parse()?,
        g.parse()?,
        b.parse()?,
      ))),
      ["for", count] => Ok(Self::For(count.parse()?)),
      ["generate"] => Ok(Self::Generate),
      ["identity"] => Ok(Self::Operation(Operation::Identity)),
      ["invert"] => Ok(Self::Operation(Operation::Invert)),
      ["load", path] => Ok(Self::Load(Some(path.parse()?))),
      ["load"] => Ok(Self::Load(None)),
      ["loop"] => Ok(Self::Loop),
      ["mod", divisor, remainder] => Ok(Self::Mask(Mask::Mod {
        divisor: divisor.parse()?,
        remainder: remainder.parse()?,
      })),
      ["open", path] => Ok(Self::Open(Some(path.parse()?))),
      ["open"] => Ok(Self::Open(None)),
      ["print"] => Ok(Self::Print),
      ["random-mask"] => Ok(Self::RandomMask),
      ["repl"] => Ok(Self::Repl),
      ["resize", cols, rows] => Ok(Self::Resize((rows.parse()?, cols.parse()?))),
      ["resize", size] => {
        let size = size.parse()?;
        Ok(Self::Resize((size, size)))
      }
      ["rotate", turns] => Ok(Self::Rotate(turns.parse()?)),
      ["rotate-color", axis, turns] => Ok(Self::Operation(Operation::RotateColor(
        axis.parse()?,
        turns.parse()?,
      ))),
      ["rows", nrows, step] => Ok(Self::Mask(Mask::Rows {
        nrows: nrows.parse()?,
        step: step.parse()?,
      })),
      ["save", path] => Ok(Self::Save(Some(path.parse()?))),
      ["save"] => Ok(Self::Save(None)),
      ["scale", scaling] => Ok(Self::Scale(scaling.parse()?)),
      ["seed", seed] => Ok(Self::Seed(seed.parse()?)),
      ["square"] => Ok(Self::Mask(Mask::Square)),
      ["top"] => Ok(Self::Mask(Mask::Top)),
      ["verbose"] => Ok(Self::Verbose),
      ["wrap"] => Ok(Self::Wrap),
      ["x"] => Ok(Self::Mask(Mask::X)),
      _ => Err(format!("Invalid command: {}", s).into()),
    }
  }
}
