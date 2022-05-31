use super::*;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Command {
  Alpha(f64),
  Apply,
  Choose(Vec<Command>),
  Default(Vector3<u8>),
  Mask(Mask),
  Operation(Operation),
  Rotate(f64),
  Scale(f64),
  Seed(u64),
  Wrap,
}

impl Command {
  pub(crate) fn parse_program(program: &str) -> Result<Vec<Command>> {
    let mut commands = Vec::new();

    for line in program.lines() {
      let source = line
        .split_once('#')
        .map(|(command, _comment)| command)
        .unwrap_or(line)
        .trim();

      for command in source.split(';') {
        let command = command.trim();

        if command.is_empty() {
          continue;
        }

        commands.push(command.parse()?);
      }
    }

    Ok(commands)
  }
}

impl FromStr for Command {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.split_whitespace().collect::<Vec<&str>>().as_slice() {
      ["all"] => Ok(Self::Mask(Mask::All)),
      ["alpha", alpha] => Ok(Self::Alpha(alpha.parse()?)),
      ["apply"] => Ok(Self::Apply),
      ["choose", words @ ..] => Ok(Self::Choose(
        words
          .iter()
          .cloned()
          .map(Command::from_str)
          .collect::<Result<Vec<Command>>>()?,
      )),
      ["circle"] => Ok(Self::Mask(Mask::Circle)),
      ["cross"] => Ok(Self::Mask(Mask::Cross)),
      ["debug"] => Ok(Self::Operation(Operation::Debug)),
      ["default", r, g, b] => Ok(Self::Default(Vector3::new(
        r.parse()?,
        g.parse()?,
        b.parse()?,
      ))),
      ["identity"] => Ok(Self::Operation(Operation::Identity)),
      ["invert"] => Ok(Self::Operation(Operation::Invert)),
      ["mod", divisor, remainder] => Ok(Self::Mask(Mask::Mod {
        divisor: divisor.parse()?,
        remainder: remainder.parse()?,
      })),
      ["rotate", turns] => Ok(Self::Rotate(turns.parse()?)),
      ["rotate-color", axis, turns] => Ok(Self::Operation(Operation::RotateColor(
        axis.parse()?,
        turns.parse()?,
      ))),
      ["rows", nrows, step] => Ok(Self::Mask(Mask::Rows {
        nrows: nrows.parse()?,
        step: step.parse()?,
      })),
      ["scale", scaling] => Ok(Self::Scale(scaling.parse()?)),
      ["seed", seed] => Ok(Self::Seed(seed.parse()?)),
      ["square"] => Ok(Self::Mask(Mask::Square)),
      ["top"] => Ok(Self::Mask(Mask::Top)),
      ["wrap"] => Ok(Self::Wrap),
      ["x"] => Ok(Self::Mask(Mask::X)),
      _ => Err(format!("Invalid command: {}", s).into()),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn case(program: &str, expected_commands: &[Command]) {
    assert_eq!(Command::parse_program(program).unwrap(), expected_commands);
  }

  #[test]
  fn semicolons_can_be_used_to_separate_commands() {
    case("apply;apply", &[Command::Apply, Command::Apply]);
  }

  #[test]
  fn leading_blank_lines_are_ignored() {
    case("\napply", &[Command::Apply]);
  }

  #[test]
  fn trailing_blank_lines_are_ignored() {
    case("apply\n", &[Command::Apply]);
  }

  #[test]
  fn intermediate_blank_lines_are_ignored() {
    case("apply\n\napply", &[Command::Apply, Command::Apply]);
  }

  #[test]
  fn extra_whitespace_between_arguments_is_ignored() {
    case("scale  0.5", &[Command::Scale(0.5)]);
  }

  #[test]
  fn leading_whitespace_is_ignored() {
    case("  apply", &[Command::Apply]);
  }

  #[test]
  fn trailing_whitespace_is_ignored() {
    case("apply  ", &[Command::Apply]);
  }

  #[test]
  fn comments_are_ignored() {
    case("# foo", &[]);
  }

  #[test]
  fn empty_lines_with_extra_whitespace_are_ignored() {
    case("  ", &[]);
  }
}
