use super::*;

#[derive(Clone, Debug)]
pub(crate) enum Command {
  Alpha(f64),
  Apply,
  Autosave,
  Comment,
  Default(Vector3<u8>),
  For(usize),
  Load(Option<PathBuf>),
  Loop,
  Mask(Mask),
  Open(Option<PathBuf>),
  Operation(Operation),
  Print,
  RandomMask,
  Read,
  Repl,
  Resize((usize, usize)),
  Rotate(f64),
  Save(Option<PathBuf>),
  Scale(f64),
  Seed(u64),
  Verbose,
  Window,
  Wrap,
}

impl FromStr for Command {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.split(':').collect::<Vec<&str>>().as_slice() {
      ["all"] => Ok(Self::Mask(Mask::All)),
      ["alpha", alpha] => Ok(Self::Alpha(alpha.parse()?)),
      ["apply"] => Ok(Self::Apply),
      ["autosave"] => Ok(Self::Autosave),
      ["circle"] => Ok(Self::Mask(Mask::Circle)),
      ["comment", ..] => Ok(Self::Comment),
      ["cross"] => Ok(Self::Mask(Mask::Cross)),
      ["default", r, g, b] => Ok(Self::Default(Vector3::new(
        r.parse()?,
        g.parse()?,
        b.parse()?,
      ))),
      ["for", count] => Ok(Self::For(count.parse()?)),
      ["identity"] => Ok(Self::Operation(Operation::Identity)),
      ["invert"] => Ok(Self::Operation(Operation::Invert)),
      ["load", path] => Ok(Self::Load(Some(path.parse()?))),
      ["load"] => Ok(Self::Load(None)),
      ["loop"] => Ok(Self::Loop),
      ["mod", divisor, remainder] => Ok(Self::Mask(Mask::Mod {
        divisor: divisor.parse()?,
        remainder: remainder.parse()?,
      })),
      ["open", path] => Ok(Self::Open(Some(path.parse()?))),
      ["open"] => Ok(Self::Open(None)),
      ["print"] => Ok(Self::Print),
      ["random-mask"] => Ok(Self::RandomMask),
      ["read"] => Ok(Self::Read),
      ["repl"] => Ok(Self::Repl),
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
      ["top"] => Ok(Self::Mask(Mask::Top)),
      ["verbose"] => Ok(Self::Verbose),
      ["window"] => Ok(Self::Window),
      ["wrap"] => Ok(Self::Wrap),
      ["x"] => Ok(Self::Mask(Mask::X)),
      _ => Err(format!("Invalid command: {}", s).into()),
    }
  }
}
