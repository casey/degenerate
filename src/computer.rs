use super::*;

const DEFAULT_OUTPUT_PATH: &str = "memory.png";

pub(crate) struct Computer {
  alpha: f64,
  autosave: bool,
  default: Vector3<u8>,
  frame: u64,
  loop_counter: usize,
  mask: Mask,
  memory: DMatrix<Vector3<u8>>,
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
  pub(crate) fn run(display: &Display, words: impl IntoIterator<Item = impl AsRef<str>>) -> Result {
    let mut computer = Self::new();

    computer.resize(display.dimensions()?);

    for arg in words {
      computer.program.push(arg.as_ref().parse()?);
    }

    while let Some(command) = computer.program.get(computer.program_counter).cloned() {
      if computer.verbose {
        eprintln!(
          "PC {} LC {} M {:?} C {:?}",
          computer.program_counter, computer.loop_counter, computer.mask, command,
        );
      }
      computer.execute(command)?;
      computer.program_counter = computer.program_counter.wrapping_add(1);
    }

    display.render(&computer.memory)?;

    Ok(())
  }

  pub(crate) fn step(&mut self, display: &Display) -> Result {
    while let Some(command) = self.program.get(self.program_counter).cloned() {
      if self.verbose {
        eprintln!(
          "PC {} LC {} M {:?} C {:?}",
          self.program_counter, self.loop_counter, self.mask, command,
        );
      }
      self.execute(command.clone())?;
      self.program_counter = self.program_counter.wrapping_add(1);

      if command == Command::Apply {
        display.render(&self.memory)?;
        break;
      }
    }

    Ok(())
  }

  pub(crate) fn done(&self) -> bool {
    self.program_counter >= self.program.len()
  }

  pub(crate) fn with_program(program: &[Command]) -> Self {
    Self {
      alpha: 1.0,
      autosave: false,
      default: Vector3::new(0, 0, 0),
      frame: 0,
      loop_counter: 0,
      mask: Mask::All,
      memory: DMatrix::zeros(0, 0),
      operation: Operation::Invert,
      program: program.into(),
      program_counter: 0,
      rng: StdRng::seed_from_u64(0),
      similarity: Similarity2::identity(),
      verbose: false,
      wrap: false,
      viewport: Viewport::Fill,
    }
  }

  pub(crate) fn new() -> Self {
    Self::with_program(&[])
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
    let mut output = self.memory.clone();
    for col in 0..self.memory.ncols() {
      for row in 0..self.memory.nrows() {
        let i = Vector2::new(col, row);
        let v = self.viewport.coordinates(dimensions, i);
        let v = similarity * v;
        let v = if self.wrap { v.wrap() } else { v };
        let i = self.viewport.pixel(dimensions, v);
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
          let over = self.operation.apply(v, input);
          let over = over.map(|c| c as f64);
          let under = self.memory[(row, col)];
          let under = under.map(|c| c as f64);
          let combined =
            (over * self.alpha + under * (1.0 - self.alpha)) / (self.alpha + (1.0 - self.alpha));
          output[(row, col)] = combined.map(|c| c as u8);
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
        self.default = default;
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
      Command::RandomMask => {
        let mask = self.rng.gen();
        self.execute(Command::Mask(mask))?;
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
      Command::Save(path) => self.image()?.save(
        path
          .as_deref()
          .unwrap_or_else(|| DEFAULT_OUTPUT_PATH.as_ref()),
      )?,
      Command::Scale(scaling) => {
        self.similarity.append_scaling_mut(scaling);
      }
      Command::Seed(seed) => self.rng = StdRng::seed_from_u64(seed),
      Command::Verbose => self.verbose = !self.verbose,
      Command::Wrap => self.wrap = !self.wrap,
    }

    Ok(())
  }

  fn resize(&mut self, dimensions: (usize, usize)) {
    self
      .memory
      .resize_mut(dimensions.0, dimensions.1, self.default)
  }

  fn image(&self) -> Result<RgbImage> {
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
          element.map(|scalar| scalar as u32).sum() / (16 * 3)
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
      .as_rgb8()
      .ok_or_else(|| format!("{} is not a valid rgb8 image", path.display()))?
      .to_owned();

    let (width, height) = (image.width() as usize, image.height() as usize);

    self.memory = DMatrix::from_iterator(
      width,
      height,
      image
        .rows()
        .flat_map(|row| row.map(|pixel| Vector3::new(pixel[0], pixel[1], pixel[2]))),
    )
    .transpose();

    Ok(())
  }
}
