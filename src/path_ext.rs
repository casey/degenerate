use super::*;

pub(crate) trait PathExt {
  fn expand(&self) -> Result<PathBuf>;
}

impl PathExt for Path {
  fn expand(&self) -> Result<PathBuf> {
    Ok(PathBuf::from(
      tilde(
        self
          .to_str()
          .ok_or_else(|| format!("Path was not valid unicode: {}", self.display()))?,
      )
      .to_string(),
    ))
  }
}
