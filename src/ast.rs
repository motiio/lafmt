use crate::tokenizer::Token;
use derive_more::Display;

#[derive(Debug, Display)]
#[display(fmt = "{:?}", token_type)]
pub struct ASTNode<'a> {
    pub token_type: &'a Token<'a>,
}
