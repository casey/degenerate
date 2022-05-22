use super::*;

#[derive(Copy, Clone)]
pub(crate) struct Token<'src> {
  kind: TokenKind,
  lexeme: &'src str,
}

impl<'src> Token<'src> {
  pub(crate) fn new(kind: TokenKind, lexeme: &'src str) -> Self {
    Self { kind, lexeme }
  }

  pub(crate) fn kind(self) -> TokenKind {
    self.kind
  }

  pub(crate) fn lexeme(self) -> &'src str {
    self.lexeme
  }
}
