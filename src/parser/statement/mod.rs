mod query;
use crate::parser::Parser;
pub use query::DMLParseError;
use query::{parse_query_block, parse_subquery, SubQuery};

#[derive(Debug)]
pub enum Statement {
    // SELECT
    Query(SubQuery),
}

pub fn parse_query_statement(p: &mut Parser) -> Statement {
    let s_q = parse_subquery(p).expect("Jopa").unwrap();
    Statement::Query(s_q)
}
