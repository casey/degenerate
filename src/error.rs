use super::*;

// Use gloo error?

#[derive(Debug)]
pub(crate) enum Error {
  Js(JsValue),
  Rust(Box<dyn std::error::Error>),
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::Rust(e) => write!(f, "{}", e),
      Self::Js(e) => write!(f, "{:?}", e),
    }
  }
}

impl From<JsValue> for Error {
  fn from(e: JsValue) -> Self {
    Self::Js(e)
  }
}

impl From<String> for Error {
  fn from(e: String) -> Self {
    Self::Rust(e.into())
  }
}

impl From<&str> for Error {
  fn from(e: &str) -> Self {
    Self::Rust(e.to_string().into())
  }
}

impl From<ImageError> for Error {
  fn from(e: ImageError) -> Self {
    Self::Rust(e.into())
  }
}

impl From<serde_json::Error> for Error {
  fn from(e: serde_json::Error) -> Self {
    Self::Rust(e.into())
  }
}

impl From<std::convert::Infallible> for Error {
  fn from(_: Infallible) -> Self {
    unreachable!();
  }
}

impl From<TryFromIntError> for Error {
  fn from(e: TryFromIntError) -> Self {
    Self::Rust(e.into())
  }
}

impl From<FromHexError> for Error {
  fn from(e: FromHexError) -> Self {
    Self::Rust(e.into())
  }
}

impl From<Utf8Error> for Error {
  fn from(e: Utf8Error) -> Self {
    Self::Rust(e.into())
  }
}

impl std::error::Error for Error {}
