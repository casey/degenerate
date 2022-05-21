use super::*;

pub(crate) fn window() -> Window {
  web_sys::window().expect("`window` missing")
}
