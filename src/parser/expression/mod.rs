#[derive(Debug)]
pub enum Expression {
    SimpleExpression(SimpleExpression),
}

#[derive(Debug)]
pub struct SimpleExpression {
    pub column: String,
    pub schema: Option<String>,
    pub table: Option<String>,
}
