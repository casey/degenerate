#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) enum TokenKind {
  Comment,
  Eof,
  Identifier,
  Newline,
  Number,
  Whitespace,
}
