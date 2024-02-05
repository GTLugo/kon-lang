#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword {
  If,
  Else,
  For,
  While,
  Loop,
  Return,
  _Self,
  _SelfType,
  Super,
  Export,
  Import,
  Public,
  Type,
  Impl,
  As,
  Trait,
}

impl Keyword {
  pub const AS: &'static str = "as";
  pub const ELSE: &'static str = "else";
  pub const EXPORT: &'static str = "export";
  pub const FOR: &'static str = "for";
  pub const IF: &'static str = "if";
  pub const IMPL: &'static str = "impl";
  pub const IMPORT: &'static str = "import";
  pub const LOOP: &'static str = "loop";
  pub const PUBLIC: &'static str = "pub";
  pub const RETURN: &'static str = "return";
  pub const SELF: &'static str = "self";
  pub const SELF_TYPE: &'static str = "Self";
  pub const SUPER: &'static str = "super";
  pub const TRAIT: &'static str = "trait";
  pub const TYPE: &'static str = "type";
  pub const WHILE: &'static str = "while";

  pub fn lexeme(&self) -> String {
    match self {
      Keyword::If => Keyword::IF.into(),
      Keyword::Else => Keyword::ELSE.into(),
      Keyword::For => Keyword::FOR.into(),
      Keyword::While => Keyword::WHILE.into(),
      Keyword::Loop => Keyword::LOOP.into(),
      Keyword::Return => Keyword::RETURN.into(),
      Keyword::_Self => Keyword::SELF.into(),
      Keyword::_SelfType => Keyword::SELF_TYPE.into(),
      Keyword::Super => Keyword::SUPER.into(),
      Keyword::Export => Keyword::EXPORT.into(),
      Keyword::Import => Keyword::IMPORT.into(),
      Keyword::Public => Keyword::PUBLIC.into(),
      Keyword::Type => Keyword::TYPE.into(),
      Keyword::Impl => Keyword::IMPL.into(),
      Keyword::As => Keyword::AS.into(),
      Keyword::Trait => Keyword::TRAIT.into(),
    }
  }
}
