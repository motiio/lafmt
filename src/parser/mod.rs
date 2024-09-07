// use crate::AstNode;
mod expression;
mod statement;

use self::statement::{parse_query_statement, DMLParseError, Statement};

use crate::tokenizer::{self, keyword::Keyword, Token};
use std::iter::Iterator;
use std::ops::{Range, RangeInclusive, RangeTo};
use std::slice::SliceIndex;

#[derive(Debug)]
pub struct Column {
    pub value: String,
    pub alias: Option<String>,
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
    // pub ast: AstNode,
}

#[derive(Debug)]
pub enum ParserError {
    UnknownParsersError(String),
    DMLParseError(DMLParseError),
}

macro_rules! impl_from_error {
    ($error_type:ty, $variant:path) => {
        impl From<$error_type> for ParserError {
            fn from(error: $error_type) -> Self {
                $variant(error)
            }
        }
    };
}
impl_from_error!(DMLParseError, ParserError::DMLParseError);

impl Parser {
    pub fn new() -> Parser {
        Parser {
            tokens: vec![],
            index: 0,
        }
    }

    pub fn parse(&mut self, query: &str) -> Result<Vec<Statement>, ParserError> {
        let mut result: Vec<Statement> = vec![];
        if let Ok(tokens) = tokenizer::tokenize(query) {
            self.tokens = tokens;
        }
        // Start parsing tokens
        let mut curr_token = self.peek_token().unwrap_or_else(|| &Token::Semicolon);
        loop {
            match curr_token {
                Token::EOF => break,
                Token::Semicolon => continue,
                Token::Keyword(Keyword::Select) => {
                    let q = parse_query_statement(self);
                    result.push(q);
                }
                _ => {
                    // Копируем значение self.index до вызова изменяемых методов
                    break;
                }
            }

            // После этого безопасно вызвать изменяющий метод
            curr_token = self.token_next().unwrap_or_else(|| &Token::Semicolon);
        }
        Ok(result)
    }

    pub fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    pub fn next_token(&mut self) -> Option<&Token> {
        //++token
        self.index += 1;
        self.tokens.get(self.index)
    }

    pub fn token_next(&mut self) -> Option<&Token> {
        //token++
        let token = self.tokens.get(self.index);
        self.index += 1;
        token
    }

    pub fn prev_token(&mut self) -> Option<&Token> {
        self.index -= 1;
        self.tokens.get(self.index)
    }

    pub fn peek_nth(&self, index: usize) -> Option<&Token> {
        self.tokens.get(self.index + index)
    }

    pub fn move_index(&mut self, size: usize) -> usize{
        self.index += size;
        self.index
    }
}
