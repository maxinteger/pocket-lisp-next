use crate::token::TokenType::String;
use crate::token::{Token, TokenType};

pub struct Scanner<'a> {
    source: &'a str,
    source_len: usize,
    chars: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}

fn is_symbol(ch: char) -> bool {
    matches!(
        ch,
        '=' | '+' | '-' | '*' | '/' | '\\' | '&' | '%' | '$' | '_' | '!' | '<' | '>' | '?' | '\''
    )
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        let chars = source.chars().collect::<Vec<char>>();
        let source_len = chars.len();
        Scanner {
            source,
            chars,
            source_len,
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
            ':' => self.keyword(),
            c if c.is_digit(10) || c == '-' && self.peek().is_digit(10) => self.number(),
            c if c.is_ascii_alphanumeric() || is_symbol(c) => self.identifier(),
            '"' => self.string(),
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            '[' => self.make_token(TokenType::LeftSquare),
            ']' => self.make_token(TokenType::RightSquare),
            '#' => self.make_token(TokenType::Dispatch),

            _ => {
                println!("SCANNER {}", c);
                self.error_token("Unexpected character.")
            }
        };
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.chars[self.current - 1]
    }

    fn advance_while_digits(&mut self) {
        while !self.is_at_end() && self.peek().is_digit(10) {
            self.advance();
        }
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
                ' ' | '\r' | '\t' | ',' => {
                    self.advance();
                }
                ';' => {
                    if self.peek_next() == '#' {
                        self.advance(); // ;
                        self.advance(); // #
                        while !self.is_at_end() && self.peek() != '#' && self.peek_next() != ';' {
                            self.advance();
                        }
                        self.advance(); // #
                        self.advance(); // ;
                    } else {
                        while !self.is_at_end() && self.peek() != '\n' {
                            self.advance();
                        }
                    }
                }
                _ => return,
            }
        }
    }

    #[inline]
    fn is_at_end(&self) -> bool {
        self.current == self.source_len
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
        self.start = self.current;
        while !self.is_at_end() && self.peek() != '"' {
            // todo add string escape?
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }
        if self.is_at_end() {
            self.error_token("Unterminated string")
        } else {
            let token = self.make_token(TokenType::String);
            self.advance();
            token
        }
    }

    fn number(&mut self) -> Token<'a> {
        self.advance_while_digits();

        if !self.is_at_end() {
            return match self.peek() {
                '.' => {
                    self.advance();
                    self.advance_while_digits();
                    self.make_token(TokenType::FloatNumber)
                }
                '/' => {
                    if !self.peek_next().is_digit(10) {
                        return self.error_token("Unterminated fraction number");
                    }
                    self.advance();
                    self.advance_while_digits();
                    self.make_token(TokenType::FractionNumber)
                }
                _ => self.make_token(TokenType::IntegerNumber),
            };
        }

        self.make_token(TokenType::IntegerNumber)
    }

    fn identifier(&mut self) -> Token<'a> {
        while !self.is_at_end() && (self.peek().is_ascii_alphanumeric() || is_symbol(self.peek())) {
            self.advance();
        }
        let token = self.identifier_type();
        self.make_token(token)
    }

    fn keyword(&mut self) -> Token<'a> {
        let peek = self.peek();
        if self.is_at_end() || !peek.is_ascii_alphanumeric() {
            return self.make_token(TokenType::Identifier);
        }
        while !self.is_at_end() && self.peek().is_ascii_alphanumeric() {
            self.advance();
        }
        return self.make_token(TokenType::Keyword);
    }

    fn identifier_type(&mut self) -> TokenType {
        match self.chars[self.start] {
            _ if self.check_keyword(0, 5, "false") => TokenType::False,
            _ if self.check_keyword(0, 4, "true") => TokenType::True,
            _ => TokenType::Identifier,
        }
    }

    fn check_keyword(&self, start: usize, len: usize, rest: &str) -> bool {
        self.current - self.start == start + len
            && self.source[self.start + start..self.current] == *rest
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner::Scanner;
    use crate::token::TokenType;

    fn test_tokens(scanner: &mut Scanner, values: Vec<&str>, tokens: Vec<TokenType>) {
        for i in 0..values.len() {
            let result = scanner.scan_token();
            assert_eq!(result.kind, *tokens.get(i).unwrap());
            assert_eq!(result.src, *values.get(i).unwrap());
        }
    }

    #[test]
    fn scan_empty_source() {
        let source = "";
        let mut scanner = Scanner::new(source);
        let result = scanner.scan_token();
        assert_eq!(result.kind, TokenType::Eof);
    }

    #[test]
    fn scan_unexpected_character() {
        let source = "~";
        let mut scanner = Scanner::new(source);

        let result = scanner.scan_token();
        assert_eq!(result.kind, TokenType::Error);
        assert_eq!(result.src, "Unexpected character.");

        let result = scanner.scan_token();
        assert_eq!(result.kind, TokenType::Eof);
    }

    #[test]
    fn scan_whitespaces() {
        let source = "   \n\n ; comment\n; comment two";
        let mut scanner = Scanner::new(source);
        let result = scanner.scan_token();
        assert_eq!(result.kind, TokenType::Eof);
    }

    #[test]
    fn scan_multiline_comment() {
        let source = ";# comment line1\n line2\n line3 #;";
        let mut scanner = Scanner::new(source);
        let result = scanner.scan_token();
        assert_eq!(result.kind, TokenType::Eof);
    }

    #[test]
    fn scan_identifier() {
        let ids = vec![
            "x1", "_", "_a", "hello", "=", "+", "-", "*", "/", "\\", "&", "%", "$", "_", "!", "<",
            ">", "?", "'",
        ];
        let tokens: Vec<TokenType> = std::iter::repeat(TokenType::Identifier)
            .take(ids.len())
            .collect();
        let source = ids.join(" ");
        let mut scanner = Scanner::new(source.as_str());

        test_tokens(&mut scanner, ids, tokens);
        let result = scanner.scan_token();
        assert_eq!(result.kind, TokenType::Eof);
    }

    #[test]
    fn scan_keyword() {
        let ids = vec![":keyword", ":120", ":0Hello"];
        let tokens: Vec<TokenType> = std::iter::repeat(TokenType::Keyword)
            .take(ids.len())
            .collect();
        let source = ids.join(" ");
        let mut scanner = Scanner::new(source.as_str());

        test_tokens(&mut scanner, ids, tokens);
        let result = scanner.scan_token();
        assert_eq!(result.kind, TokenType::Eof);
    }

    #[test]
    fn scan_value_identifiers() {
        use TokenType::*;
        let ids = vec!["true", "false"];
        let tokens = vec![True, False];
        let source = ids.join(" ");
        let mut scanner = Scanner::new(source.as_str());

        test_tokens(&mut scanner, ids, tokens);
        let result = scanner.scan_token();
        assert_eq!(result.kind, TokenType::Eof);
    }

    #[test]
    fn scan_symbols() {
        use TokenType::*;
        let ids = vec!["(", ")", "{", "}", "[", "]", "#"];
        let tokens = vec![
            LeftParen,
            RightParen,
            LeftBrace,
            RightBrace,
            LeftSquare,
            RightSquare,
            Dispatch,
        ];
        let source = ids.join(" ");
        let mut scanner = Scanner::new(source.as_str());

        test_tokens(&mut scanner, ids, tokens);
        let result = scanner.scan_token();
        assert_eq!(result.kind, TokenType::Eof);
    }

    #[test]
    fn scan_number() {
        let nums = vec!["-42", "-1.5", "0", "42", "42.5", "1/3"];
        let tokens = vec![
            TokenType::IntegerNumber,
            TokenType::FloatNumber,
            TokenType::IntegerNumber,
            TokenType::IntegerNumber,
            TokenType::FloatNumber,
            TokenType::FractionNumber,
        ];
        let source = nums.join(" ");
        let mut scanner = Scanner::new(source.as_str());

        test_tokens(&mut scanner, nums, tokens);
        let result = scanner.scan_token();
        assert_eq!(result.kind, TokenType::Eof);
    }

    #[test]
    fn scan_string() {
        let cases = vec!["\"\"", "\"hello world\"", "\"multi\nline\nstring\n\""];
        let tokens = vec![TokenType::String, TokenType::String, TokenType::String];
        let source = cases.join(" ");
        let mut scanner = Scanner::new(source.as_str());

        for i in 0..cases.len() {
            let result = scanner.scan_token();
            let case = *cases.get(i).unwrap();
            assert_eq!(result.kind, *tokens.get(i).unwrap());
            assert_eq!(*result.src, case[1..case.len() - 1]); // remove quotes
        }

        let result = scanner.scan_token();
        assert_eq!(result.kind, TokenType::Eof);
    }

    #[test]
    fn scan_invalid_string() {
        let cases = vec!["\"Invalid string"];
        let source = cases.join(" ");
        let mut scanner = Scanner::new(source.as_str());

        let result = scanner.scan_token();
        assert_eq!(result.kind, TokenType::Error);
        assert_eq!(result.src, "Unterminated string");

        let result = scanner.scan_token();
        assert_eq!(result.kind, TokenType::Eof);
    }

    #[test]
    fn scan_lines() {
        let source = "\"multi\nline\nstring\n\"";
        let mut scanner = Scanner::new(source);

        let result = scanner.scan_token();
        assert_eq!(result.kind, TokenType::String);
        assert_eq!(result.src, "multi\nline\nstring\n");
        assert_eq!(result.line, 4);

        let result = scanner.scan_token();
        assert_eq!(result.kind, TokenType::Eof);
        assert_eq!(result.line, 4);
    }
}
