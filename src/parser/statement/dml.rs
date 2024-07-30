use super::Statement;
use crate::parser::catalog_info::ColumnAccess;
use crate::parser::Parser;

pub struct Select {
    pub projections: Vec<ColumnAccess>,
}
pub enum DMLParseError {
    SelectParseError(String),
}

pub fn parse_dml_statement(p: &mut Parser) -> Result<Statement, DMLParseError> {
    Err(DMLParseError::SelectParseError("Jopa".to_string()))
}
