use super::*;

const ALPHA_OPAQUE: f32 = 1.0;

pub(crate) struct Computer {
  alpha: f32,
  default: Vector4<f32>,
  gpu: Arc<Mutex<Gpu>>,
  mask: Mask,
  operation: Operation,
  program: String,
  program_counter: usize,
  rng: StdRng,
  transform: Similarity2<f32>,
  wrap: bool,
}

impl Computer {
  pub(crate) fn alpha(&self) -> f32 {
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

  pub(crate) fn transform(&self) -> &Similarity2<f32> {
    &self.transform
  }

  pub(crate) fn new(gpu: Arc<Mutex<Gpu>>) -> Self {
    Self {
      alpha: 1.0,
      default: Vector4::new(0.0, 0.0, 0.0, ALPHA_OPAQUE),
      gpu,
      mask: Mask::All,
      operation: Operation::Invert,
      program: String::new(),
      program_counter: 0,
      rng: StdRng::seed_from_u64(0),
      transform: Similarity2::identity(),
      wrap: false,
    }
  }
}
