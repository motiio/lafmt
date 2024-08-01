use super::Statement;
use crate::parser::catalog_info::ColumnAccess;
use crate::parser::{Ident, Parser};

use crate::tokenizer::{self, keyword::Keyword, Token};

#[derive(Debug)]
pub struct SubQuery {
    pub sub_query: Option<Box<SubQuery>>,
    pub query_block: Option<Box<QueryBlock>>,
}

#[derive(Debug)]
pub struct QueryBlock {
    with: Option<Box<With>>,
    distinct: Distinc,
    select_list: Vec<ColumnAccess>,
    // TODO: MUST BE NOT OPTION
    from: Option<Box<From>>,
    // where: Option<Box<Condition>>,
}

#[derive(Debug)]
struct With {
    name: String,
    aliases: Vec<ColumnAccess>,
    subquery: Box<SubQuery>,
}

#[derive(Debug)]
enum Distinc {
    Distinct,
    Unique,
    All,
}

#[derive(Debug)]
enum From {
    TableReference(TableReference),
    Join(Join),
}

#[derive(Debug)]
enum TableReference {
    QueryTableExpression(QueryTableExpression),
}

#[derive(Debug)]
struct QueryTableExpression {
    schema: Option<Box<Ident>>,
    table: Option<Box<Ident>>,
    // db_link: Option<Box<DbLink>>,
    alias: Option<Box<Ident>>,
}

#[derive(Debug)]
struct Join {
    table_reference: TableReference,
    join: Option<JoinClause>,
}

#[derive(Debug)]
enum JoinClause {
    InnerJoin(InnerJoin),
    OuterJoin(OuterJoin),
}

#[derive(Debug)]
struct InnerJoin {
    inner_type: InnerJoinType,
    join_table: TableReference,
    // using: Vec<ColumnAccess>,
    // condition: Box<Condition>,
}

#[derive(Debug)]
enum InnerJoinType {
    Inner,
    Cross,
    Natural,
}

#[derive(Debug)]
struct OuterJoin {
    outer_type: OuterJoinType,
    join_table: TableReference,
    // using: Vec<ColumnAccess>,
    // condition: Box<Condition>,
}

#[derive(Debug)]
enum OuterJoinType {
    Left,
    Right,
    Full,
}

#[derive(Debug)]
pub struct Select {
    pub distinct: bool,
    pub select_list: Vec<ColumnAccess>,
}
#[derive(Debug)]
pub enum DMLParseError {
    SelectParseError(String),
    PareseSubQueryError(String),
}
pub fn parse_subquery(p: &mut Parser) -> Result<Option<SubQuery>, DMLParseError> {
    let token = p.curr_token();
    match token {
        Some(Token::LParen) => {
            let _ = p.fetch_token();
            let subquery = SubQuery {
                sub_query: parse_subquery(p)?.map(Box::new),
                query_block: None,
            };
            Ok(Some(subquery))
        }
        Some(Token::Keyword(Keyword::Select)) | Some(Token::Keyword(Keyword::With)) => {
            let subquery = SubQuery {
                sub_query: None,
                query_block: Some(Box::new(parse_query_block(p)?)),
            };
            Ok(Some(subquery))
        }
        _ => Ok(None),
    }
}

pub fn parse_query_block(p: &mut Parser) -> Result<QueryBlock, DMLParseError> {
    // let with = parse_with(p)?.map(Box::new);
    let (distinct, select_list) = parse_select_list(p)?;
    // let from = parse_from(p)?.map(Box::new);
    Ok(
        QueryBlock{
            with: None,
            distinct,
            select_list,
            from: None
        }
    )
    // Err(DMLParseError::SelectParseError("jopa".to_string()))
}

fn parse_select_list(p: &mut Parser) -> Result<(Distinc, Vec<ColumnAccess>), DMLParseError> {
    let mut distinct = Distinc::All;
    let mut select_list = vec![];
    let mut list_token = p.next_token().unwrap_or_else(|| Token::EOF);
    loop {
        match list_token {
            Token::Keyword(Keyword::From) => break,
            Token::Comma => {}
            Token::Keyword(Keyword::Distinct) | Token::Keyword(Keyword::Unique) => {
                distinct = Distinc::Distinct;
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
