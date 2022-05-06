use crate::token::Token;
use crate::TokenType;

pub struct Scanner<'a> {
    source: &'a str,
    chars: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source,
            chars: source.chars().collect::<Vec<char>>(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token<'a> {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        let c = self.advance();

        return match c {
            c if c == '_' || c.is_ascii_alphabetic() => self.identifier(),
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),
            '!' => {
                let token = if self.match_current('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.make_token(token)
            }
            '=' => {
                let token = if self.match_current('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.make_token(token)
            }
            '<' => {
                let token = if self.match_current('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.make_token(token)
            }
            '>' => {
                let token = if self.match_current('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.make_token(token)
            }
            '"' => self.string(),
            c if c.is_digit(10) => self.number(),
            _ => self.error_token("Unexpected character."),
        };
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.chars[self.current - 1]
    }

    fn peek(&mut self) -> char {
        self.chars[self.current]
    }

    fn peek_next(&mut self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.chars[self.current + 1]
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while !self.is_at_end() && self.peek() != '\n' {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => return,
            }
        }
    }

    fn match_current(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.chars[self.current] != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn is_at_end(&self) -> bool {
        self.current == self.source.len()
    }

    fn make_token(&self, token_type: TokenType) -> Token<'a> {
        Token::new(
            token_type,
            self.start,
            &self.source[self.start..self.current],
            self.line,
        )
    }

    fn error_token(&self, msg: &'static str) -> Token<'a> {
        Token::new(TokenType::Error, 0, msg, self.line)
    }

    fn string(&mut self) -> Token<'a> {
        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }
        if self.is_at_end() {
            self.error_token("Unterminated string")
        } else {
            self.advance();
            self.make_token(TokenType::String)
        }
    }

    fn number(&mut self) -> Token<'a> {
        while !self.is_at_end() && self.peek().is_digit(10) {
            self.advance();
        }

        if !self.is_at_end() && self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while !self.is_at_end() && self.peek().is_digit(10) {
                self.advance();
            }
        }

        self.make_token(TokenType::Number)
    }

    fn identifier(&mut self) -> Token<'a> {
        while !self.is_at_end() && (self.peek().is_ascii_alphanumeric() || self.peek() == '_') {
            self.advance();
        }
        let token = self.identifier_type();
        self.make_token(token)
    }

    fn identifier_type(&mut self) -> TokenType {
        match self.chars[self.start] {
            'a' => self.check_keyword(1, 2, "nd", TokenType::And),
            'c' => self.check_keyword(1, 4, "lass", TokenType::Class),
            'e' => self.check_keyword(1, 3, "lse", TokenType::Else),
            'f' => {
                if self.current - self.start > 1 {
                    match self.chars[self.start + 1] {
                        'a' => self.check_keyword(2, 3, "lse", TokenType::False),
                        'o' => self.check_keyword(2, 1, "r", TokenType::For),
                        'u' => self.check_keyword(2, 1, "n", TokenType::Fun),
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            }
            'i' => self.check_keyword(1, 1, "f", TokenType::If),
            'n' => self.check_keyword(1, 2, "il", TokenType::Nil),
            'o' => self.check_keyword(1, 1, "r", TokenType::Or),
            'p' => self.check_keyword(1, 4, "rint", TokenType::Print),
            'r' => self.check_keyword(1, 5, "eturn", TokenType::Return),
            's' => self.check_keyword(1, 4, "uper", TokenType::Super),
            't' => {
                if self.current - self.start > 1 {
                    match self.chars[self.start + 1] {
                        'h' => self.check_keyword(2, 2, "is", TokenType::This),
                        'r' => self.check_keyword(2, 2, "ue", TokenType::True),
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            }
            'v' => self.check_keyword(1, 2, "ar", TokenType::Var),
            'w' => self.check_keyword(1, 4, "hile", TokenType::While),
            _ => TokenType::Identifier,
        }
    }

    fn check_keyword(
        &self,
        start: usize,
        len: usize,
        rest: &str,
        token_type: TokenType,
    ) -> TokenType {
        if self.current - self.start == start + len
            && self.source[self.start + start..self.current] == *rest
        {
            token_type
        } else {
            TokenType::Identifier
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Scanner, TokenType};

    #[test]
    fn scan_empty_source() {
        let source = "";
        let mut scanner = Scanner::new(source);
        let result = scanner.scan_token();
        assert_eq!(result.kind, TokenType::Eof);
    }

    #[test]
    fn scan_unexpected_character() {
        let source = "#";
        let mut scanner = Scanner::new(source);

        let result = scanner.scan_token();
        assert_eq!(result.kind, TokenType::Error);
        assert_eq!(result.src, "Unexpected character.");

        let result = scanner.scan_token();
        assert_eq!(result.kind, TokenType::Eof);
    }

    #[test]
    fn scan_whitespaces() {
        let source = "   \n\n // comment\n// comment two";
        let mut scanner = Scanner::new(source);
        let result = scanner.scan_token();
        assert_eq!(result.kind, TokenType::Eof);
    }

    #[test]
    fn scan_identifier() {
        let ids = ["_", "_a", "x1", "hello"];
        let source = ids.join(" ");
        let mut scanner = Scanner::new(source.as_str());
        for i in 0..ids.len() {
            let result = scanner.scan_token();
            assert_eq!(result.kind, TokenType::Identifier);
            assert_eq!(result.src, ids[i]);
        }
        let result = scanner.scan_token();
        assert_eq!(result.kind, TokenType::Eof);
    }

    #[test]
    fn scan_keywords() {
        use TokenType::*;
        let ids = [
            "and", "class", "else", "false", "for", "fun", "if", "nil", "or", "print", "return",
            "super", "this", "true", "var", "while",
        ];
        let tokens = [
            And, Class, Else, False, For, Fun, If, Nil, Or, Print, Return, Super, This, True, Var,
            While,
        ];
        let source = ids.join(" ");
        let mut scanner = Scanner::new(source.as_str());
        for i in 0..ids.len() {
            let result = scanner.scan_token();
            assert_eq!(result.kind, tokens[i]);
            assert_eq!(result.src, ids[i]);
        }
        let result = scanner.scan_token();
        assert_eq!(result.kind, TokenType::Eof);
    }

    #[test]
    fn scan_symbols() {
        use TokenType::*;
        let ids = [
            "(", ")", "{", "}", ";", ",", ".", "-", "+", "/", "*", "!", "!=", "=", "==", "<", "<=",
            ">", ">=",
        ];
        let tokens = [
            LeftParen,
            RightParen,
            LeftBrace,
            RightBrace,
            Semicolon,
            Comma,
            Dot,
            Minus,
            Plus,
            Slash,
            Star,
            Bang,
            BangEqual,
            Equal,
            EqualEqual,
            Less,
            LessEqual,
            Greater,
            GreaterEqual,
        ];
        let source = ids.join(" ");
        let mut scanner = Scanner::new(source.as_str());
        for i in 0..ids.len() {
            let result = scanner.scan_token();
            assert_eq!(result.kind, tokens[i]);
            assert_eq!(result.src, ids[i]);
        }
        let result = scanner.scan_token();
        assert_eq!(result.kind, TokenType::Eof);
    }

    #[test]
    fn scan_number() {
        let nums = ["0", "42", "42.5"];
        let source = nums.join(" ");
        let mut scanner = Scanner::new(source.as_str());
        for i in 0..nums.len() {
            let result = scanner.scan_token();
            assert_eq!(result.kind, TokenType::Number);
            assert_eq!(result.src, nums[i]);
        }
        let result = scanner.scan_token();
        assert_eq!(result.kind, TokenType::Eof);
    }

    #[test]
    fn scan_string() {
        let nums = [
            "\"\"",
            "\"hello world\"",
            "\"multi\nline\nstring\n\"",
            "\"Invalid string",
        ];
        let source = nums.join(" ");
        let mut scanner = Scanner::new(source.as_str());
        for i in 0..(nums.len() - 1) {
            let result = scanner.scan_token();
            assert_eq!(result.kind, TokenType::String);
            assert_eq!(result.src, nums[i]);
        }

        let result = scanner.scan_token();
        assert_eq!(result.kind, TokenType::Error);
        assert_eq!(result.src, "Unterminated string");

        let result = scanner.scan_token();
        assert_eq!(result.kind, TokenType::Eof);
    }

    #[test]
    fn scan_lines() {
        let source = "var \"multi\nline\nstring\n\"";
        let mut scanner = Scanner::new(source);
        let result = scanner.scan_token();
        assert_eq!(result.kind, TokenType::Var);
        assert_eq!(result.line, 1);

        let result = scanner.scan_token();
        assert_eq!(result.kind, TokenType::String);
        assert_eq!(result.line, 4);

        let result = scanner.scan_token();
        assert_eq!(result.kind, TokenType::Eof);
        assert_eq!(result.line, 4);
    }
}
