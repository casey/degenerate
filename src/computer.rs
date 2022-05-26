use super::*;

const ALPHA_OPAQUE: u8 = 255;

pub(crate) struct Computer {
  pub(crate) alpha: f64,
  pub(crate) default: Vector4<u8>,
  pub(crate) gpu: Arc<Mutex<Gpu>>,
  pub(crate) loop_counters: Vec<u64>,
  pub(crate) mask: Mask,
  pub(crate) operation: Operation,
  pub(crate) program: Vec<Command>,
  pub(crate) program_counter: usize,
  pub(crate) rng: StdRng,
  pub(crate) similarity: Similarity2<f64>,
  pub(crate) wrap: bool,
}

impl Default for Computer {
  fn default() -> Self {
    Self {
      alpha: 1.0,
      default: Vector4::new(0, 0, 0, ALPHA_OPAQUE),
      gpu: Arc::new(Mutex::new(Gpu::new().unwrap())),
      loop_counters: Vec::new(),
      mask: Mask::All,
      operation: Operation::Invert,
      program: Vec::new(),
      program_counter: 0,
      rng: StdRng::seed_from_u64(0),
      similarity: Similarity2::identity(),
      wrap: false,
    }
  }
}

impl Computer {
  pub(crate) fn run(&mut self, incremental: bool) -> Result {
    while let Some(command) = self.program.get(self.program_counter).cloned() {
      self.program_counter = self.program_counter.wrapping_add(1);
      self.execute(command.clone())?;

      if incremental && command == Command::Apply {
        break;
      }
    }

    Ok(())
  }

  pub(crate) fn alpha(&self) -> f64 {
    self.alpha
  }

  pub(crate) fn wrap(&self) -> bool {
    self.wrap
  }

  pub(crate) fn done(&self) -> bool {
    self.program_counter >= self.program.len()
  }

  pub(crate) fn load_program(&mut self, program: &[Command]) {
    self.program = program.into();
    self.program_counter = 0;
  }

  pub(crate) fn program(&self) -> &[Command] {
    &self.program
  }

  pub(crate) fn mask(&self) -> &Mask {
    &self.mask
  }

  pub(crate) fn operation(&self) -> &Operation {
    &self.operation
  }

  pub(crate) fn similarity(&self) -> &Similarity2<f64> {
    &self.similarity
  }

  pub(crate) fn new(gpu: Arc<Mutex<Gpu>>) -> Self {
    Self {
      alpha: 1.0,
      default: Vector4::new(0, 0, 0, ALPHA_OPAQUE),
      gpu,
      loop_counters: Vec::new(),
      mask: Mask::All,
      operation: Operation::Invert,
      program: Vec::new(),
      program_counter: 0,
      rng: StdRng::seed_from_u64(0),
      similarity: Similarity2::identity(),
      wrap: false,
    }
  }

  fn apply(&mut self) -> Result {
    self.gpu.lock().unwrap().apply(self)?;
    Ok(())
  }

  fn execute(&mut self, command: Command) -> Result<()> {
    match command {
      Command::Alpha(alpha) => self.alpha = alpha,
      Command::Apply => self.apply()?,
      Command::Choose(commands) => {
        if let Some(command) = commands.choose(&mut self.rng) {
          self.execute(command.clone())?;
        }
      }
      Command::Default(default) => {
        self.default = Vector4::new(default.x, default.y, default.z, ALPHA_OPAQUE);
      }
      Command::For(until) => {
        if until == 0 {
          while let Some(command) = self.program.get(self.program_counter) {
            self.program_counter += 1;

            if let Command::Loop = command {
              break;
            }
          }
        } else {
          self.loop_counters.push(until);
        }
      }
      Command::Loop => match self.loop_counters.last_mut() {
        Some(loop_counter) => {
          if *loop_counter > 1 {
            *loop_counter -= 1;
            let mut skip = 0;
            self.program_counter -= 2;
            while let Some(command) = self.program.get(self.program_counter) {
              match command {
                Command::For(_) => {
                  if skip > 0 {
                    skip -= 1;
                  } else {
                    self.program_counter += 1;
                    break;
                  }
                }
                Command::Loop => skip += 1,
                _ => {}
              }
              self.program_counter -= 1;
            }
          } else {
            self.loop_counters.pop();
          }
        }
        None => self.program_counter = 0,
      },
      Command::Mask(mask) => self.mask = mask,
      Command::Operation(operation) => self.operation = operation,
      Command::Rotate(turns) => self
        .similarity
        .append_rotation_mut(&UnitComplex::from_angle(turns * f64::consts::TAU)),
      Command::Scale(scaling) => {
        self.similarity.append_scaling_mut(scaling);
      }
      Command::Seed(seed) => self.rng = StdRng::seed_from_u64(seed),
      Command::Wrap => self.wrap = !self.wrap,
    }

    Ok(())
  }

  pub(crate) fn resize(&mut self) -> Result {
    self.gpu.lock().unwrap().resize()?;
    Ok(())
  }
}
