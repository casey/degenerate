use {super::*, TokenKind::*};

struct Lexer<'src> {
  src: &'src str,
  token_end: usize,
  token_start: usize,
  tokens: Vec<Token<'src>>,
}

impl<'src> Lexer<'src> {
  fn lex(src: &'src str) -> Result<Vec<Token<'src>>> {
    Lexer {
      src,
      token_end: 0,
      token_start: 0,
      tokens: Vec::new(),
    }
    .lex_program()
  }

  fn lex_program(mut self) -> Result<Vec<Token<'src>>> {
    while let Some(c) = self.next() {
      self.lex_token(c)?;
    }

    Ok(self.tokens)
  }

  fn lex_token(&mut self, first: char) -> Result {
    match first {
      ' ' | '\t' => self.lex_whitespace(),
      '#' => self.lex_comment(),
      '0'..='9' => self.lex_number()?,
      '\n' => {
        self.advance();
        self.token(Newline);
      }
      _ if Self::is_identifier_start(first) => self.lex_identifier(),
      _ => return Err(format!("Unknown start of token: `{}`", first).into()),
    }

    Ok(())
  }

  fn lex_comment(&mut self) {
    loop {
      if let None | Some('\n') = self.next() {
        break;
      }
      self.advance();
    }
    self.token(TokenKind::Comment);
  }

  fn lex_whitespace(&mut self) {
    while let Some(' ' | '\t') = self.next() {
      self.advance();
    }
    self.token(TokenKind::Whitespace);
  }

  fn advance(&mut self) {
    self.token_end += self.next().unwrap().len_utf8();
  }

  fn next(&self) -> Option<char> {
    self.src[self.token_end..].chars().next()
  }

  fn lex_identifier(&mut self) {
    self.advance();

    loop {
      match self.next() {
        None => break,
        Some(c) => {
          if !Self::is_identifier_continue(c) {
            break;
          }
        }
      }
      self.advance();
    }

    self.token(TokenKind::Identifier);
  }

  fn is_identifier_start(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_')
  }

  fn is_identifier_continue(c: char) -> bool {
    Self::is_identifier_start(c) || matches!(c, '0'..='9')
  }

  fn lex_number(&mut self) -> Result {
    let mut integer = 0;
    let mut fraction = 0;
    let mut dot = false;

    loop {
      match self.next() {
        None => break,
        Some('0'..='9') => {
          if dot {
            fraction += 1;
          } else {
            integer += 1;
          }
        }
        Some('.') => {
          if dot == true {
            panic!();
          }
          dot = true;
        }
        Some(_) => break,
      }
      self.advance();
    }

    if integer == 0 {
      return Err("Number must have integer part".into());
    }

    if dot && fraction == 0 {
      return Err("Number must have fraction part".into());
    }

    self.token(TokenKind::Number);

    Ok(())
  }

  fn token(&mut self, kind: TokenKind) {
    assert!(self.token_end > self.token_start);

    self.tokens.push(Token::new(
      kind,
      &self.src[self.token_start..self.token_end],
    ));

    self.token_start = self.token_end;
  }
}

#[cfg(test)]
mod test {
  use super::*;

  fn case(program: &str, expected_tokens: &[TokenKind]) {
    let actual_tokens = Lexer::lex(program)
      .unwrap()
      .iter()
      .cloned()
      .map(Token::kind)
      .collect::<Vec<TokenKind>>();
    assert_eq!(actual_tokens, expected_tokens);
  }

  #[test]
  fn spaces_are_whitespace() {
    case(" ", &[Whitespace]);
  }

  #[test]
  fn tabs_are_whitespace() {
    case("\t", &[Whitespace]);
  }

  #[test]
  fn multiple_whitespace_characters_produce_one_token() {
    case(" \t", &[Whitespace]);
    case("\t ", &[Whitespace]);
  }

  #[test]
  fn hashes_introduce_comments() {
    case("#", &[Comment]);
  }

  #[test]
  fn end_of_program_terminates_comments() {
    case("#   ", &[Comment]);
  }

  #[test]
  fn newline_terminates_comments() {
    case("#    \n", &[Comment, Newline]);
  }

  #[test]
  fn identifier() {
    case("foo_100", &[Identifier]);
  }

  #[test]
  fn number() {
    case("0", &[Number]);
    case("00000", &[Number]);
    case("0.0", &[Number]);
    case("00000.00000", &[Number]);
  }

  #[test]
  fn integer_part_must_precede_dot() {
    assert!(Lexer::lex(".0").is_err());
  }

  #[test]
  fn fractional_part_must_follow_dot() {
    assert!(Lexer::lex("0.").is_err());
  }

  #[test]
  fn dot_is_invalid() {
    assert!(Lexer::lex(".").is_err());
  }
}
