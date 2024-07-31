use super::{Query, QueryBody, Statement};
use crate::parser::catalog_info::ColumnAccess;
use crate::parser::{Ident, Parser};

use crate::tokenizer::{self, keyword::Keyword, Token};

#[derive(Debug)]
pub struct Select {
    pub distinct: bool,
    pub select_list: Vec<ColumnAccess>,
}
#[derive(Debug)]
pub enum DMLParseError {
    SelectParseError(String),
    ProjectionParseError(String),
    UnexpectedDMLToken(String),
}

pub fn parse_dml_statement(p: &mut Parser) -> Result<Statement, DMLParseError> {
    println!("From dml:{:?}-{:?}", p.curr_token(), p.index);
    // let mut with:
    // let with = if let with = Token::Keyword(Keyword::With) {
    // Some(
    // with
    // )
    // } else {
    // None
    // };
    let body = match p.curr_token() {
        Some(Token::Keyword(Keyword::Select)) => parse_select(p),
        _ => Err(DMLParseError::UnexpectedDMLToken("Jopa".to_string())),
    };

    println!("From dml:{:?}", body);
    Ok(Statement::Query(Box::new(Query {
        body: Box::new(body?),
    })))
}

fn parse_select(p: &mut Parser) -> Result<QueryBody, DMLParseError> {
    let (distinct, select_list) = parse_select_list(p)?;
    let query_body = QueryBody::Select(Box::new(Select {
        distinct,
        select_list,
    }));
    Ok(query_body)
}

fn parse_select_list(p: &mut Parser) -> Result<(bool, Vec<ColumnAccess>), DMLParseError> {
    let mut distinct = false;
    let mut select_list = vec![];
    let mut list_token = p.next_token().unwrap_or_else(|| Token::EOF);
    println!("Parse slect list: {:?}", list_token);
    loop {
        match list_token {
            Token::Keyword(Keyword::From) => break,
            Token::Comma => {}
            Token::Keyword(Keyword::Distinct) => {
                distinct = true;
            }
            Token::Asterisk(_) => select_list.push(ColumnAccess {
                column: Ident {
                    value: "*".to_string(),
                },
                alias: None,
            }),
            Token::Identifier(ref val) => select_list.push(ColumnAccess {
                column: Ident { value: val.clone() },
                alias: Some(Ident {
                    value: "Kek".to_string(),
                }),
            }),
            Token::NumberLiteral(ref value) | Token::StringLiteral(ref value) => {
                select_list.push(ColumnAccess {
                    column: Ident {
                        value: value.clone(),
                    },
                    alias: Some(Ident {
                        value: "Kek".to_string(),
                    }),
                })
            }
            _ => break,
        }
        list_token = p.next_token().unwrap_or_else(|| Token::EOF);
    }

    Ok((distinct, select_list))
}

fn parse_select_item(p: &mut Parser) -> Result<ColumnAccess, DMLParseError> {
    Err(DMLParseError::SelectParseError(
        "Not implemented".to_string(),
    ))
}
