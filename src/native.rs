use {
  super::*,
  ansi_term::{Colour::Red, Style},
};

pub(crate) fn run() {
  if let Err(error) = run_inner() {
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

fn run_inner() -> Result {
  let program = env::args()
    .skip(1)
    .into_iter()
    .map(|word| word.parse())
    .collect::<Result<Vec<Command>>>()?;

  let mut computer = Computer::new();

  computer.resize((256, 256));
  computer.load_program(&program);
  computer.run(false)?;

  Ok(())
}
