use super::*;

const ALPHA_OPAQUE: u8 = 255;

pub(crate) struct Computer {
  alpha: f64,
  default: Vector4<u8>,
  gpu: Option<Arc<Mutex<Gpu>>>,
  loop_counters: Vec<u64>,
  mask: Mask,
  memory: DMatrix<Vector4<u8>>,
  operation: Operation,
  program: Vec<Command>,
  program_counter: usize,
  rng: StdRng,
  similarity: Similarity2<f64>,
  wrap: bool,
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

  pub(crate) fn memory(&self) -> &DMatrix<Vector4<u8>> {
    &self.memory
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

  pub(crate) fn new(gpu: Option<Arc<Mutex<Gpu>>>) -> Self {
    Self {
      alpha: 1.0,
      default: Vector4::new(0, 0, 0, ALPHA_OPAQUE),
      gpu,
      loop_counters: Vec::new(),
      mask: Mask::All,
      memory: DMatrix::zeros(0, 0),
      operation: Operation::Invert,
      program: Vec::new(),
      program_counter: 0,
      rng: StdRng::seed_from_u64(0),
      similarity: Similarity2::identity(),
      wrap: false,
    }
  }

  pub(crate) fn size(&self) -> usize {
    self.memory.ncols()
  }

  fn apply(&mut self) -> Result {
    if let Some(gpu) = self.gpu.as_ref() {
      gpu.lock().unwrap().apply(self)?;
    } else {
      let similarity = self.similarity.inverse();
      let size = self.size();
      let transform = self.transform();
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
          if self.mask.is_masked(size, i, v) {
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
            let combined = over * self.alpha + under * (1.0 - self.alpha);
            output[(row, col)] = Vector4::new(
              combined.x.ceil() as u8,
              combined.y.ceil() as u8,
              combined.z.ceil() as u8,
              ALPHA_OPAQUE,
            );
          }
        }
      }
      self.memory = output;
    }

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

  pub(crate) fn resize(&mut self, size: usize) -> Result {
    if let Some(gpu) = self.gpu.as_ref() {
      gpu.lock().unwrap().resize()?;
    } else {
      self.memory.resize_mut(size, size, self.default);
    }

    Ok(())
  }

  fn transform(&self) -> Affine2<f64> {
    Affine2::from_matrix_unchecked(
      Matrix3::identity()
        .append_translation(&Vector2::from_element(0.5))
        .append_scaling(2.0 / self.size() as f64)
        .append_translation(&Vector2::from_element(-1.0))
        .append_nonuniform_scaling(&Vector2::new(1.0, -1.0)),
    )
  }
}
