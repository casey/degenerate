use super::*;

const ALPHA_OPAQUE: u8 = 255;

pub(crate) struct Computer {
  alpha: f64,
  default: Vector4<u8>,
  gpu: Arc<Mutex<Gpu>>,
  mask: Mask,
  operation: Operation,
  program: String,
  program_counter: usize,
  rng: StdRng,
  similarity: Similarity2<f64>,
  wrap: bool,
}

impl Computer {
  pub(crate) fn alpha(&self) -> f64 {
    self.alpha
  }

  pub(crate) fn wrap(&self) -> bool {
    self.wrap
  }

  pub(crate) fn load_program(&mut self, program: String) {
    self.program = program;
  }

  pub(crate) fn program(&self) -> String {
    self.program.clone()
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
      mask: Mask::All,
      operation: Operation::Invert,
      program: String::new(),
      program_counter: 0,
      rng: StdRng::seed_from_u64(0),
      similarity: Similarity2::identity(),
      wrap: false,
    }
  }

  fn apply(&mut self) -> Result {
    self.gpu.lock().unwrap().apply(self)
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
    self.gpu.lock().unwrap().resize()
  }
}
