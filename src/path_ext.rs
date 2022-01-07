use super::*;

pub(crate) trait PathExt {
  fn expand(&self) -> Result<PathBuf>;
}

impl PathExt for Path {
  fn expand(&self) -> Result<PathBuf> {
    match self.starts_with("~/") {
      true => Ok(
        home_dir()
          .unwrap_or_default()
          .join(self.strip_prefix("~/")?),
      ),
      false => Ok(self.to_path_buf()),
    }
  }
}
