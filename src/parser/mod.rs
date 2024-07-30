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
        let next_token = self.next_token().unwrap_or_else(|| Token::Semicolon);
        println!("{:?}", next_token);
        loop {
            match next_token {
                Token::EOF => break,
                Token::Semicolon => continue,
                Token::Keyword(Keyword::Select) => {
                    self.prev_token();
                    let q = parse_dml_statement(self)?;
                    result.push(q);
                }
                _ => {
                    println!("{:?}", next_token);
                    break;
                }
            }
        }

        // println!("{:?}", self.tokens);
        Ok(result)
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.index).map(|token| token.clone());
        self.index += 1;
        token
    }
    pub fn prev_token(&mut self) -> Option<Token> {
        self.index -= 1;
        let token = self.tokens.get(self.index).map(|token| token.clone());
        token
    }
}
