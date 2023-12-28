use std::iter::Peekable;
use std::str::Chars;

pub struct CharacterProvider<'a> {
  chars: Peekable<Chars<'a>>,
  line: u32,
  column: u32,
}

impl<'a> CharacterProvider<'a> {
  pub fn new(source: &'a str) -> Self {
    Self {
      chars: source.chars().peekable(),
      line: 1,
      column: 1,
    }
  }

  pub fn current_line(&self) -> u32 {
    self.line
  }

  pub fn current_column(&self) -> u32 {
    self.column
  }

  pub fn peek(&mut self) -> Option<&char> {
    self.chars.peek()
  }

  fn filter_whitespace(&mut self, ch: char) -> Option<char> {
    let mut ch = ch;

    if ch.is_whitespace() {
      while ch.is_whitespace() {
        if ch == '\n' {
          self.line += 1;
          self.column = 1;
        } else {
          self.column += 1;
        }
        ch = self.chars.next()?;
      }
    }

    Some(ch)
  }

  fn filter_comments(&mut self, ch: char) -> Option<char> {
    let mut ch = ch;

    if ch == '/' {
      if let Some(peek) = self.chars.peek() {
        if *peek == '/' {
          while ch != '\n' {
            self.column += 1;
            ch = self.chars.next()?;
          }
          self.line += 1;
          self.column = 1;
          ch = self.chars.next()?;
        }
      }
    }

    Some(ch)
  }
}

impl Iterator for CharacterProvider<'_> {
  type Item = char;

  fn next(&mut self) -> Option<Self::Item> {
    let ch = self.chars.next()?;
    self.column += 1;

    let ch = self.filter_whitespace(ch)?;
    let ch = self.filter_comments(ch)?;
    let ch = self.filter_whitespace(ch)?;

    Some(ch)
  }
}
