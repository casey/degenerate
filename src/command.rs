use super::*;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Command {
  Alpha(f64),
  Apply,
  Choose(Vec<Command>),
  Comment,
  Default(Vector3<u8>),
  For(u64),
  Loop,
  Mask(Mask),
  Operation(Operation),
  Resize((u64, u64)),
  Rotate(f64),
  Save(Option<PathBuf>),
  Scale(f64),
  Seed(u64),
  Viewport(Viewport),
  Wrap,
}

impl FromStr for Command {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.split(':').collect::<Vec<&str>>().as_slice() {
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
      ["comment", ..] => Ok(Self::Comment),
      ["cross"] => Ok(Self::Mask(Mask::Cross)),
      ["debug"] => Ok(Self::Operation(Operation::Debug)),
      ["default", r, g, b] => Ok(Self::Default(Vector3::new(
        r.parse()?,
        g.parse()?,
        b.parse()?,
      ))),
      ["fit"] => Ok(Self::Viewport(Viewport::Fit)),
      ["fill"] => Ok(Self::Viewport(Viewport::Fill)),
      ["for", count] => Ok(Self::For(count.parse()?)),
      ["identity"] => Ok(Self::Operation(Operation::Identity)),
      ["invert"] => Ok(Self::Operation(Operation::Invert)),
      ["loop"] => Ok(Self::Loop),
      ["mod", divisor, remainder] => Ok(Self::Mask(Mask::Mod {
        divisor: divisor.parse()?,
        remainder: remainder.parse()?,
      })),
      ["resize", cols, rows] => Ok(Self::Resize((rows.parse()?, cols.parse()?))),
      ["resize", size] => {
        let size = size.parse()?;
        Ok(Self::Resize((size, size)))
      }
      ["rotate", turns] => Ok(Self::Rotate(turns.parse()?)),
      ["rotate-color", axis, turns] => Ok(Self::Operation(Operation::RotateColor(
        axis.parse()?,
        turns.parse()?,
      ))),
      ["rows", nrows, step] => Ok(Self::Mask(Mask::Rows {
        nrows: nrows.parse()?,
        step: step.parse()?,
      })),
      ["save", path] => Ok(Self::Save(Some(path.parse()?))),
      ["save"] => Ok(Self::Save(None)),
      ["scale", scaling] => Ok(Self::Scale(scaling.parse()?)),
      ["seed", seed] => Ok(Self::Seed(seed.parse()?)),
      ["square"] => Ok(Self::Mask(Mask::Square)),
      ["stretch"] => Ok(Self::Viewport(Viewport::Stretch)),
      ["top"] => Ok(Self::Mask(Mask::Top)),
      ["wrap"] => Ok(Self::Wrap),
      ["x"] => Ok(Self::Mask(Mask::X)),
      _ => Err(format!("Invalid command: {}", s).into()),
    }
  }
}
