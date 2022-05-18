use super::*;

const DEFAULT_OUTPUT_PATH: &str = "memory.png";
const ALPHA_OPAQUE: u8 = 255;

pub(crate) struct Computer {
  alpha: f64,
  default: Vector4<u8>,
  loop_counter: usize,
  mask: Mask,
  memory: DMatrix<Vector4<u8>>,
  operation: Operation,
  program: Vec<Command>,
  program_counter: usize,
  rng: StdRng,
  similarity: Similarity2<f64>,
  viewport: Viewport,
  wrap: bool,
}

impl Computer {
  pub(crate) fn run(&mut self, incremental: bool) -> Result {
    while let Some(command) = self.program.get(self.program_counter).cloned() {
      self.execute(command.clone())?;
      self.program_counter = self.program_counter.wrapping_add(1);

      if incremental && command == Command::Apply {
        break;
      }
    }

    Ok(())
  }

  #[cfg(target_arch = "wasm32")]
  pub(crate) fn memory(&self) -> &DMatrix<Vector4<u8>> {
    &self.memory
  }

  #[cfg(target_arch = "wasm32")]
  pub(crate) fn done(&self) -> bool {
    self.program_counter >= self.program.len()
  }

  pub(crate) fn load_program(&mut self, program: &[Command]) {
    self.program = program.into();
    self.program_counter = 0;
  }

  #[cfg(target_arch = "wasm32")]
  pub(crate) fn program(&self) -> &[Command] {
    &self.program
  }

  pub(crate) fn new() -> Self {
    Self {
      alpha: 1.0,
      default: Vector4::new(0, 0, 0, ALPHA_OPAQUE),
      loop_counter: 0,
      mask: Mask::All,
      memory: DMatrix::zeros(0, 0),
      operation: Operation::Invert,
      program: Vec::new(),
      program_counter: 0,
      rng: StdRng::seed_from_u64(0),
      similarity: Similarity2::identity(),
      wrap: false,
      viewport: Viewport::Fill,
    }
  }

  fn dimensions(&self) -> Vector2<usize> {
    Vector2::new(self.memory.ncols(), self.memory.nrows())
  }

  fn apply(&mut self) -> Result {
    let similarity = self.similarity.inverse();
    let dimensions = self.dimensions();
    let transform = self.viewport.transform(dimensions);
    let inverse = transform.inverse();
    let mut output = self.memory.clone();
    for col in 0..self.memory.ncols() {
      for row in 0..self.memory.nrows() {
        let i = Point2::new(col as f64, row as f64);
        let v = transform.transform_point(&i);
        let v = similarity.transform_point(&v);
        let v = if self.wrap { v.wrap() } else { v };
        let i = inverse
          .transform_point(&v)
          .map(|element| element.round() as isize);
        if self.mask.is_masked(dimensions, i, v) {
          let input = if i.x >= 0
            && i.y >= 0
            && i.x < self.memory.ncols() as isize
            && i.y < self.memory.nrows() as isize
          {
            self.memory[(i.y as usize, i.x as usize)]
          } else {
            self.default
          };
          let over = self.operation.apply(v, input.xyz()).map(|c| c as f64);
          let under = self.memory[(row, col)].xyz().map(|c| c as f64);
          let combined =
            (over * self.alpha + under * (1.0 - self.alpha)) / (self.alpha + (1.0 - self.alpha));
          output[(row, col)] = Vector4::new(
            combined.x as u8,
            combined.y as u8,
            combined.z as u8,
            ALPHA_OPAQUE,
          );
        }
      }
    }
    self.memory = output;
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
      Command::Comment => {}
      Command::Default(default) => {
        self.default = Vector4::new(default.x, default.y, default.z, ALPHA_OPAQUE);
      }
      Command::Viewport(viewport) => self.viewport = viewport,
      Command::For(until) => {
        if self.loop_counter as u64 >= until {
          loop {
            self.program_counter = self.program_counter.wrapping_add(1);
            if let Some(Command::Loop) | None = self.program.get(self.program_counter) {
              break;
            }
          }
          self.loop_counter = 0;
        }
      }
      Command::Loop => {
        loop {
          self.program_counter = self.program_counter.wrapping_sub(1);
          let next = self.program_counter.wrapping_add(1);
          if next == 0 {
            break;
          }
          if let Some(Command::For(_)) | None = self.program.get(next) {
            break;
          }
        }
        self.loop_counter += 1;
      }
      Command::Mask(mask) => self.mask = mask,
      Command::Operation(operation) => self.operation = operation,
      Command::Resize(dimensions) => {
        self.resize((dimensions.0.try_into()?, dimensions.1.try_into()?));
      }
      Command::Rotate(turns) => self
        .similarity
        .append_rotation_mut(&UnitComplex::from_angle(turns * f64::consts::TAU)),
      Command::Save(path) => {
        if cfg!(not(target_arch = "wasm32")) {
          self.image()?.save(
            path
              .as_deref()
              .unwrap_or_else(|| DEFAULT_OUTPUT_PATH.as_ref()),
          )?
        }
      }
      Command::Scale(scaling) => {
        self.similarity.append_scaling_mut(scaling);
      }
      Command::Seed(seed) => self.rng = StdRng::seed_from_u64(seed),
      Command::Wrap => self.wrap = !self.wrap,
    }

    Ok(())
  }

  pub(crate) fn resize(&mut self, dimensions: (usize, usize)) {
    self
      .memory
      .resize_mut(dimensions.0, dimensions.1, self.default)
  }

  fn image(&self) -> Result<RgbaImage> {
    ImageBuffer::from_raw(
      self.memory.ncols().try_into()?,
      self.memory.nrows().try_into()?,
      self.memory.transpose().iter().flatten().cloned().collect(),
    )
    .ok_or_else(|| "Memory is not a valid image".into())
  }
}
