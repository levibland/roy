use crate::ast::*;
use crate::token::*;
use std::slice::Iter;
use thiserror::Error;
use colored::*;

pub type ParseResult<T> = Result<T, ParseError>;

pub struct Parser<'p> {
    tokens: Iter<'p, Token>,
    current: Token,
    peek: Token,
}

impl<'p> Parser<'p> {
    pub fn new(tokens: Iter<'p, Token>) -> Self {
        Parser {
            tokens: tokens,
            current: Token::Eof,
            peek: Token::Eof,
        }
    }

    fn expect_token(&mut self, token: Token) -> ParseResult<Token> {
        if self.current_is(token) {
            Ok(self.current.clone())
        } else {
            Err(ParseError::UnexpectedToken(self.current.clone()))
        }
    }

    fn expect_token_and_read(&mut self, token: Token) -> ParseResult<Token> {
        let result = self.expect_token(token)?;

        self.read();

        Ok(result)
    }

    fn expect_identifier_and_read(&mut self) -> ParseResult<Token> {
        self.expect_token_and_read(Token::Identifier("".to_string()))
    }

    fn current_is(&self, token: Token) -> bool {
        std::mem::discriminant(&self.current) == std::mem::discriminant(&token)
    }

    fn read(&mut self) {
        self.current = self.peek.clone();
        self.peek = if let Some(token) = self.tokens.next() { token.clone() } else { Token::Eof };
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unexpected token {0:?}")]
    UnexpectedToken(Token),
}

impl ParseError {
    pub fn print(self) {
        eprintln!("{}", format!("{}", self).red().bold());
    }
}