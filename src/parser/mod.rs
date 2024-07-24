// use crate::AstNode;

pub struct Parser<'a> {
    pub raw_code: &'a str,
    // pub ast: AstNode,
}

impl<'a> Parser<'a> {
    pub fn new(raw_code: &'a str) -> Parser {
        Parser { raw_code }
    }
}
