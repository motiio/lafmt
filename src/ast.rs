use crate::tokenizer::Token;

pub enum DataManipulation {
    Select,
    Update,
}

// Перечисление для типов выражений
pub enum Expression {
    Literal(Token),
    Operation(Token),
}

pub enum StatementType {
    DataManipulation(DataManipulation),
}

pub struct Statement {
    stmt_type: StatementType,
    parent: Option<AstNode<Construction>>,
    children: Vec<AstNode<Construction>>,
}

pub enum Construction {
    Statement(Statement),
    Expression(Expression),
}

pub struct AstNode<Construction> {
    construction: Construction,
}

