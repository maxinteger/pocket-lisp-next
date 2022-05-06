use crate::scanner::Scanner;
use crate::token::{Token, TokenType};
use anyhow::{Error, Result};

type Program = Vec<ExpressionList>;

type ExpressionList = Vec<ExpressionNode>;

enum ExpressionNode {
    Empty,
    BooleanLiteral(bool),
    IntegerNumberLiteral(u64),
    FloatNumberLiteral(f64),
    FractionNumberLiteral(u64, u64),
    StringLiteral(String),
    Identifier(String),
    Keyword(String),
    Dispatch(ExpressionList),
    List(ExpressionList),
}

pub struct Parser<'a> {
    scanner: &'a mut Scanner<'a>,
    previous: Token<'a>,
    current: Token<'a>,
    program: Program,
    had_error: bool,
    panic_mode: bool,
    last_error: String,
}

impl<'a> Parser<'a> {
    pub fn new(scanner: &'a mut Scanner<'a>) -> Self {
        Parser {
            previous: Token::default(),
            current: Token::default(),
            scanner,
            program: vec![],
            had_error: false,
            panic_mode: false,
            last_error: "".to_owned(),
        }
    }

    pub fn parse(&mut self) -> Result<&Program> {
        self.advance();
        while !self.is_end() {
            let result = self.expression_list();
            match result {
                Ok(expression) => self.program.push(expression),
                Err(error) => self.error_at_current(error.to_string().as_str()),
            }
        }
        self.consume(TokenType::Eof, "Expect end of expression.");
        if self.had_error {
            let msg = self.last_error.clone();
            Err(Error::msg(&msg))
        } else {
            Ok(&self.program)
        }
    }

    fn expression_list(&mut self) -> Result<ExpressionList> {
        let token = self.current;
        return match token.kind {
            TokenType::LeftParen => Ok(self.advance_until(TokenType::RightParen)?),
            TokenType::LeftBrace => Ok(self.advance_until(TokenType::RightBrace)?),
            TokenType::LeftSquare => Ok(self.advance_until(TokenType::RightSquare)?),
            _ => Err(Error::msg(format!(
                "Expected (, {{ pr [, but get {}",
                token.kind
            ))),
        };
    }

    fn expression(&mut self) -> Result<ExpressionNode> {
        let token = self.current;
        return match token.kind {
            TokenType::True => Ok(ExpressionNode::BooleanLiteral(true)),
            TokenType::False => Ok(ExpressionNode::BooleanLiteral(false)),
            TokenType::String => Ok(ExpressionNode::StringLiteral(token.src.to_owned())),
            TokenType::IntegerNumber => {
                let val = token.src.parse::<u64>().expect("Integer number token");
                Ok(ExpressionNode::IntegerNumberLiteral(val))
            }
            TokenType::FloatNumber => {
                let val = token.src.parse::<f64>().expect("Float number token");
                Ok(ExpressionNode::FloatNumberLiteral(val))
            }
            TokenType::FractionNumber => {
                let a: Vec<u64> = token
                    .src
                    .split("/")
                    .map(|num| num.parse::<u64>().expect("Integer number token "))
                    .collect();
                Ok(ExpressionNode::FractionNumberLiteral(a[0], a[1]))
            }
            TokenType::Identifier => Ok(ExpressionNode::StringLiteral(token.src.to_owned())),
            TokenType::Keyword => Ok(ExpressionNode::Keyword(token.src.to_owned())),
            TokenType::Dispatch => {
                let list = self.expression_list()?;
                Ok(ExpressionNode::Dispatch(list))
            }
            TokenType::LeftParen | TokenType::LeftBrace | TokenType::LeftSquare => {
                let list = self.expression_list()?;
                Ok(ExpressionNode::List(list))
            }
            TokenType::Eof => Ok(ExpressionNode::Empty),
            _ => Err(Error::msg(format!("Unexpected token {}", token.kind))),
        };
    }

    fn advance(&mut self) {
        loop {
            self.current = self.scanner.scan_token();
            if self.current.kind == TokenType::Error {
                self.error_at_current(self.current.src);
            } else {
                break;
            }
        }
    }

    fn advance_until(&mut self, end_token: TokenType) -> Result<Vec<ExpressionNode>> {
        let mut items = vec![];
        self.advance();
        loop {
            if self.current.kind == end_token || self.is_end() {
                break;
            } else {
                let exp = self.expression()?;
                items.push(exp)
            }
        }
        if self.current.kind != end_token {
            self.error_at_current(
                format!("Expected {}, but get {}", end_token, self.current.kind).as_str(),
            )
        }
        Ok(items)
    }

    fn consume(&mut self, kind: TokenType, message: &str) {
        if self.current.kind == kind {
            self.advance()
        } else {
            self.error_at_current(message)
        }
    }

    fn peek(&self) -> Token {
        self.current
    }

    fn is_end(&self) -> bool {
        self.peek().kind == TokenType::Eof
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(self.previous, message)
    }

    fn error_at(&mut self, token: Token, message: &str) {
        if self.panic_mode {
            return;
        }

        self.panic_mode = true;
        let line_prefix = format!("[line {}] Error", token.line);

        let token = if token.kind == TokenType::Eof {
            " at end".to_owned()
        } else if token.kind == TokenType::Error {
            todo!()
        } else {
            format!(" at '{}'", token.src)
        };

        self.had_error = true;
        self.last_error = format!("{}{}: {}", line_prefix, token, message);

        eprintln!("{}", self.last_error);
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{Parser, Program};
    use crate::scanner::Scanner;
    use crate::token::TokenType;
    use anyhow::{Error, Result};

    fn parse(src: &str) -> Result<&Program> {
        let mut scanner = Scanner::new(src);
        let mut parser = Parser::new(&mut scanner);
        let res = parser.parse();
        return res.;
    }

    #[test]
    fn parse_empty_program() {
        let res = parse("").unwrap();
        let expected = vec![] as Program;
        assert_eq!(res.len(), 0)
    }
}
