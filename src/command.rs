use super::*;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Command {
  Alpha(f32),
  Default(Vector3<f32>),
  Mask(Mask),
  Operation(Operation),
  Rotate(f32),
  Scale(f32),
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
      ["circle"] => Ok(Self::Mask(Mask::Circle)),
      ["cross"] => Ok(Self::Mask(Mask::Cross)),
      ["debug"] => Ok(Self::Operation(Operation::Debug)),
      ["default", r, g, b] => Ok(Self::Default(Vector3::new(
        r.parse::<f32>()?.clamp(0.0, 1.0),
        g.parse::<f32>()?.clamp(0.0, 1.0),
        b.parse::<f32>()?.clamp(0.0, 1.0),
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
