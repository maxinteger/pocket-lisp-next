use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Ord, PartialOrd, Eq, Hash, Copy, Clone)]
pub enum TokenType {
    Init,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftSquare,
    RightSquare,
    Dispatch,
    True,
    False,
    Identifier,
    Keyword,
    String,
    IntegerNumber,
    FloatNumber,
    FractionNumber,
    Error,
    Eof,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            TokenType::Init => "Init",
            TokenType::LeftParen => "LeftParen",
            TokenType::RightParen => "RightParen",
            TokenType::LeftBrace => "LeftBrace",
            TokenType::RightBrace => "RightBrace",
            TokenType::LeftSquare => "LeftSquare",
            TokenType::RightSquare => "RightSquare",
            TokenType::Dispatch => "Dispatch",
            TokenType::True => "True",
            TokenType::False => "False",
            TokenType::Identifier => "Identifier",
            TokenType::Keyword => "Keyword",
            TokenType::String => "String",
            TokenType::IntegerNumber => "IntegerNumber",
            TokenType::FloatNumber => "FloatNumber",
            TokenType::FractionNumber => "FractionNumber",
            TokenType::Error => "Error",
            TokenType::Eof => "Eof",
        })
    }
}

#[derive(Copy, Clone)]
pub struct Token<'a> {
    pub kind: TokenType,
    pub start: usize,
    pub src: &'a str,
    pub line: usize,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenType, start: usize, src: &'a str, line: usize) -> Self {
        Token {
            kind,
            start,
            src,
            line,
        }
    }
}

impl Default for Token<'static> {
    fn default() -> Token<'static> {
        Token {
            kind: TokenType::Init,
            start: 0,
            src: "",
            line: 0,
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            "[{}] \"{}\" {}:{}",
            self.kind,
            self.src, self.line, self.start
        )
    }
}
