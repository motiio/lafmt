
pub struct Parser<'a> {
    pub raw_code: &'a str,
    pub ast: AstNode,
}

impl Parser<'a> {
    pub fn new<'a>(raw_code: &'a str, tokens: &'a Vec<Token>) -> Parser {
        Parser { raw_code, tokens }
    }
}
