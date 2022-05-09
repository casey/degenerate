use {
  super::*,
  ansi_term::{Colour::Red, Style},
  rustyline::error::ReadlineError,
};

pub(crate) fn run() {
  if let Err(error) = Computer::run(&Display, env::args().skip(1)) {
    if let Some(ReadlineError::Eof | ReadlineError::Interrupted) =
      error.downcast_ref::<ReadlineError>()
    {
      return;
    }

    if atty::is(atty::Stream::Stderr)
      || env::var("CLICOLOR_FORCE")
        .map(|val| val != "0")
        .unwrap_or_default()
    {
      eprintln!(
        "{}{}",
        Red.bold().paint("error"),
        Style::new().bold().paint(format!(": {}", error))
      );
    } else {
      eprintln!("error: {}", error);
    }

    process::exit(1);
  }
}

pub(crate) struct Display;

impl Display {
  pub(crate) fn render(&self, _memory: &DMatrix<Vector3<u8>>) {}

  pub(crate) fn dimensions(&self) -> (usize, usize) {
    (256, 256)
  }
}
