use super::*;

#[derive(Clone, Debug)]
pub(crate) enum DefaultValue {
  Color(Vector3<u8>),
  Coordinates,
}
