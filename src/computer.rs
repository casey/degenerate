use super::*;

const DEFAULT_OUTPUT_PATH: &str = "memory.png";
const ALPHA_OPAQUE: u8 = 255;

pub(crate) struct Computer {
  alpha: f64,
  autosave: bool,
  default: Vector4<u8>,
  frame: u64,
  loop_counter: usize,
  mask: Mask,
  memory: DMatrix<Vector4<u8>>,
  operation: Operation,
  program: Vec<Command>,
  program_counter: usize,
  rng: StdRng,
  similarity: Similarity2<f64>,
  verbose: bool,
  viewport: Viewport,
  wrap: bool,
}

impl Computer {
  pub(crate) fn run(&mut self, incremental: bool) -> Result {
    while let Some(command) = self.program.get(self.program_counter).cloned() {
      if self.verbose {
        eprintln!(
          "PC {} LC {} M {:?} C {:?}",
          self.program_counter, self.loop_counter, self.mask, command,
        );
      }
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
      autosave: false,
      default: Vector4::new(0, 0, 0, ALPHA_OPAQUE),
      frame: 0,
      loop_counter: 0,
      mask: Mask::All,
      memory: DMatrix::zeros(0, 0),
      operation: Operation::Invert,
      program: Vec::new(),
      program_counter: 0,
      rng: StdRng::seed_from_u64(0),
      similarity: Similarity2::identity(),
      verbose: false,
      wrap: false,
      viewport: Viewport::Fill,
    }
  }

  fn autosave(&mut self) -> Result {
    if self.autosave {
      self.image()?.save(format!("{}.png", self.frame))?;
      self.frame += 1;
    }
    Ok(())
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
    self.autosave()?;
    Ok(())
  }

  fn execute(&mut self, command: Command) -> Result<()> {
    match command {
      Command::Alpha(alpha) => self.alpha = alpha,
      Command::Apply => self.apply()?,
      Command::Autosave => self.autosave = !self.autosave,
      Command::Comment => {}
      Command::Default(default) => {
        self.default = Vector4::new(default.x, default.y, default.z, ALPHA_OPAQUE);
      }
      Command::Viewport(viewport) => self.viewport = viewport,
      Command::For(until) => {
        if self.loop_counter >= until {
          loop {
            self.program_counter = self.program_counter.wrapping_add(1);
            if let Some(Command::Loop) | None = self.program.get(self.program_counter) {
              break;
            }
          }
          self.loop_counter = 0;
        }
      }
      Command::Load(path) => {
        self.load(
          path
            .as_deref()
            .unwrap_or_else(|| DEFAULT_OUTPUT_PATH.as_ref()),
        )?;
        self.autosave()?;
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
      Command::Open(path) => {
        let command = if let Some(command) = env::var_os("DEGENERATE_OPEN_COMMAND") {
          command
        } else if cfg!(target_os = "macos") {
          "open".into()
        } else if cfg!(target_os = "linux") {
          "xdg-open".into()
        } else if cfg!(target_os = "windows") {
          "explorer".into()
        } else {
          return Err("Please supply an open command by setting the `DEGENERATE_OPEN_COMMAND` environment variable".into());
        };
        process::Command::new(command)
          .arg(
            path
              .as_deref()
              .unwrap_or_else(|| DEFAULT_OUTPUT_PATH.as_ref()),
          )
          .spawn()?;
      }
      Command::Mask(mask) => self.mask = mask,
      Command::Operation(operation) => self.operation = operation,
      Command::Print => self.print()?,
      Command::Random(commands) => {
        if let Some(command) = commands.choose(&mut self.rng) {
          self.execute(command.clone())?;
        }
      }
      Command::Read => {
        let source = fs::read_to_string("program.degen")?;

        let mut program = Vec::new();

        for word in source.split_whitespace() {
          program.push(word.parse()?);
        }

        self
          .program
          .splice(self.program_counter + 1..self.program_counter + 1, program);
      }
      #[cfg(not(target_arch = "wasm32"))]
      Command::Repl => {
        use {dirs::home_dir, rustyline::Editor};

        let history = home_dir().unwrap_or_default().join(".degenerate_history");

        let mut editor = Editor::<()>::new();
        editor.load_history(&history).ok();

        loop {
          let line = editor.readline("> ")?;

          editor.add_history_entry(line.as_str());
          editor.save_history(&history)?;

          match line.parse::<Command>() {
            Ok(command) => {
              self.execute(command)?;
              self.print()?;
            }
            Err(err) => {
              eprintln!("Could not parse command from `{}`: {}", line, err);
            }
          }
        }
      }
      Command::Resize(dimensions) => {
        self.resize(dimensions);
        self.autosave()?;
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
      Command::Verbose => self.verbose = !self.verbose,
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

  fn print(&self) -> Result<()> {
    let mut w = BufWriter::new(io::stdout());

    for row in self.memory.row_iter() {
      for element in &row {
        write!(
          w,
          "{:X}",
          element.xyz().map(|scalar| scalar as u32).sum() / (16 * 3)
        )?;
      }
      writeln!(w)?;
    }

    w.flush()?;

    Ok(())
  }

  fn load(&mut self, path: &Path) -> Result<()> {
    let image = image::io::Reader::open(path)?
      .decode()?
      .as_rgba8()
      .ok_or_else(|| format!("{} is not a valid rgb8 image", path.display()))?
      .to_owned();

    let (width, height) = (image.width() as usize, image.height() as usize);

    self.memory = DMatrix::from_iterator(
      width,
      height,
      image
        .rows()
        .flat_map(|row| row.map(|pixel| Vector4::new(pixel[0], pixel[1], pixel[2], pixel[3]))),
    )
    .transpose();

    Ok(())
  }
}
