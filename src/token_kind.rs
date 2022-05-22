#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) enum TokenKind {
  Whitespace,
  Comment,
  Identifier,
  Number,
  Newline,
}
