use crate::ast;
use crate::tokenizer::{Token, Tokenizer};

pub struct Parser<'a> {
    pub raw_code: &'a String,
    _tokenizer: Tokenizer<'a>,
}

impl Parser<'_> {
    pub fn new(raw_code: &String) -> Parser {
        Parser {
            raw_code,
            _tokenizer: Tokenizer::new(&raw_code), // formated_code: String::new(),
        }
    }

    pub fn parse(&mut self) -> ast::ASTNode {
        let vec = self._tokenizer.produce_tokens();
        ast::ASTNode {
            token_type: &vec[0],
        }
    }
    pub fn tokens(&self) -> &Vec<Token> {
        self._tokenizer.tokens()
    }
}
