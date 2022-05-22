use {super::*, TokenKind::*};

pub(crate) struct Parser<'src> {
  tokens: Vec<Token<'src>>,
  next: usize,
}

impl<'src> Parser<'src> {
  pub(crate) fn parse(src: &str) -> Result<Vec<Command>> {
    let mut tokens = Lexer::lex(src)?;
    tokens.retain(|token| !matches!(token.kind(), Whitespace | Comment));
    tokens.push(Token::new(Eof, ""));
    Parser { tokens, next: 0 }.parse_program()
  }

  fn parse_program(mut self) -> Result<Vec<Command>> {
    let mut commands = Vec::new();

    loop {
      match self.next() {
        Newline => self.advance(),
        Eof => break,
        Whitespace | Comment => unreachable!(),
        _ => commands.push(self.parse_command()?),
      }
    }

    assert_eq!(self.next(), Eof);
    assert_eq!(self.next, self.tokens.len() - 1);

    Ok(commands)
  }

  fn parse_command(&mut self) -> Result<Command> {
    match self.expect(Identifier) {
      "all" => Ok(Command::Mask(Mask::All)),
      "alpha" => Ok(Command::Alpha(self.parse_number())),
      "apply" => Ok(Command::Apply),
      "choose" => {
        let mut choices = Vec::new();
        loop {
          if let Eof | Newline = self.next() {
            break;
          }
          choices.push(self.parse_command()?);
        }
        Ok(Command::Choose(choices))
      }
      "circle" => Ok(Command::Mask(Mask::Circle)),
      "cross" => Ok(Command::Mask(Mask::Cross)),
      "debug" => Ok(Command::Operation(Operation::Debug)),
      "default" => {
        let r = self.parse_number();
        let g = self.parse_number();
        let b = self.parse_number();
        Ok(Command::Default(Vector3::new(r as u8, g as u8, b as u8)))
      }
      _ => panic!(),
    }
  }

  fn parse_number(&mut self) -> f64 {
    let lexeme = self.expect(Number);
    lexeme.parse().unwrap()
  }

  fn expect(&mut self, kind: TokenKind) -> &'src str {
    if self.next() != kind {
      todo!()
    }

    self.presume(kind)
  }

  fn presume(&mut self, kind: TokenKind) -> &'src str {
    assert_eq!(self.next(), kind);
    let lexeme = self.tokens[self.next].lexeme();
    self.next += 1;
    lexeme
  }

  fn next(&self) -> TokenKind {
    self.tokens[self.next].kind()
  }

  fn advance(&mut self) {
    assert!(self.next < self.tokens.len());
    self.next += 1;
  }
}

#[cfg(test)]
mod test {
  use super::*;

  fn case(program: &str, expected_commands: &[Command]) {
    assert_eq!(Parser::parse(program).unwrap(), expected_commands);
  }

  #[test]
  fn apply() {
    case("apply", &[Command::Apply]);
  }

  #[test]
  fn all() {
    case("all", &[Command::Mask(Mask::All)]);
  }

  #[test]
  fn alpha() {
    case("alpha 1.0", &[Command::Alpha(1.0)]);
  }

  #[test]
  fn choose_zero() {
    case("choose", &[Command::Choose(Vec::new())]);
  }

  #[test]
  fn choose_one() {
    case("choose apply", &[Command::Choose(vec![Command::Apply])]);
  }

  #[test]
  fn choose_two() {
    case(
      "choose apply apply",
      &[Command::Choose(vec![Command::Apply, Command::Apply])],
    );
  }

  #[test]
  fn circle() {
    case("circle", &[Command::Mask(Mask::Circle)]);
  }

  #[test]
  fn cross() {
    case("cross", &[Command::Mask(Mask::Cross)]);
  }

  #[test]
  fn debug() {
    case("debug", &[Command::Operation(Operation::Debug)]);
  }

  #[test]
  fn default() {
    case(
      "default 255 255 255",
      &[Command::Default(Vector3::new(255, 255, 255))],
    );
  }

  #[test]
  fn command_with_newline() {
    case("apply\n", &[Command::Apply]);
  }

  #[test]
  fn whitespace_is_ignored() {
    case(" ", &[]);
  }

  #[test]
  fn comments_are_ignored() {
    case("# foo", &[]);
  }

  #[test]
  fn blank_lines_are_ignored() {
    case("\n\n\n", &[]);
  }
}
