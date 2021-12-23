use {
  super::*,
  winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
  },
};

#[derive(Clone, Debug)]
pub(crate) enum Command {
  Filter(Filter),
  For(usize),
  Loop,
  Operation(Operation),
  Print,
  Repl,
  Resize((usize, usize)),
  Save(PathBuf),
  Verbose,
  Window,
}

impl Command {
  pub(crate) fn apply(&self, state: &mut State) -> Result<()> {
    match self {
      Self::Filter(filter) => {
        for col in 0..state.matrix.ncols() {
          for row in 0..state.matrix.nrows() {
            if filter.filter(state, row, col) {
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
        for result in BufReader::new(io::stdin()).lines() {
          let line = result?;
          match line.trim().parse::<Self>() {
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
      Self::Save(path) => state.image()?.save(path)?,
      Self::Verbose => state.verbose = !state.verbose,
      Self::Window => {
        let mut event_loop = EventLoop::new();
        let window = Window::new(&event_loop)?;

        let mut state = state.clone();
        state.program_counter = state.program_counter.wrapping_add(1);

        event_loop.run(move |event, _, control_flow| match event {
          Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
          } => *control_flow = ControlFlow::Exit,
          _ => match state.step() {
            Err(error) => {
              eprintln!("error: {}", error);
              *control_flow = ControlFlow::Exit;
            }
            Ok(true) => {}
            Ok(false) => {
              *control_flow = ControlFlow::Exit;
            }
          },
        });
      }
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
      ["for", count] => Ok(Self::For(count.parse()?)),
      ["invert"] => Ok(Self::Operation(Operation::Invert)),
      ["loop"] => Ok(Self::Loop),
      ["mod", divisor, remainder] => Ok(Self::Filter(Filter::Mod {
        divisor: divisor.parse()?,
        remainder: remainder.parse()?,
      })),
      ["print"] => Ok(Self::Print),
      ["random"] => Ok(Self::Operation(Operation::Random)),
      ["repl"] => Ok(Self::Repl),
      ["resize", cols, rows] => Ok(Self::Resize((rows.parse()?, cols.parse()?))),
      ["save", path] => Ok(Self::Save(path.parse()?)),
      ["square"] => Ok(Self::Filter(Filter::Square)),
      ["top"] => Ok(Self::Filter(Filter::Top)),
      ["verbose"] => Ok(Self::Verbose),
      ["window"] => Ok(Self::Window),
      _ => Err(format!("Invalid command: {}", s).into()),
    }
  }
}
