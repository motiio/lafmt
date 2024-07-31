// use crate::AstNode;
mod catalog_info;
mod statement;

use self::statement::{parse_dml_statement, DMLParseError, Statement};

use crate::tokenizer::{self, keyword::Keyword, Token};

#[derive(Debug)]
pub struct Ident {
    pub value: String,
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
        let mut curr_token = self.curr_token().unwrap_or_else(|| Token::Semicolon);
        loop {
            match curr_token {
                Token::EOF => break,
                Token::Semicolon => continue,
                Token::Keyword(Keyword::Select) => {
                    let q = parse_dml_statement(self)?;
                    result.push(q);
                }
                _ => {
                    println!("{:?} at Index[{}]", curr_token, self.index);
                    break;
                }
            }

            curr_token = self.next_token().unwrap_or_else(|| Token::Semicolon);
        }
        Ok(result)
    }
    pub fn curr_token(&self) -> Option<Token> {
        let token = self.tokens.get(self.index).map(|token| token.clone());
        token
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.index += 1;
        let token = self.tokens.get(self.index).map(|token| token.clone());
        token
    }
    pub fn prev_token(&mut self) -> Option<Token> {
        self.index -= 1;
        let token = self.tokens.get(self.index).map(|token| token.clone());
        token
    }
}
