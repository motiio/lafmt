use super::Statement;
use crate::parser::expression::{Expression, SimpleExpression};
use crate::parser::{Column, Parser};
use std::ops::{Range, RangeInclusive};

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
    select_list: Vec<SelectItem>,
    // TODO: MUST BE NOT OPTION
    from: Option<Box<From>>,
    // where: Option<Box<Condition>>,
}

#[derive(Debug)]
struct With {
    name: String,
    aliases: Vec<SelectItem>,
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
    schema: Option<String>,
    table: Option<String>,
    // db_link: Option<Box<DbLink>>,
    alias: Option<String>,
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
    pub select_list: Vec<SelectItem>,
}

#[derive(Debug)]
struct SelectItem {
    expression: Expression,
    alias: Option<String>,
}
#[derive(Debug)]
pub enum DMLParseError {
    SelectParseError(String),
    PareseSubQueryError(String),
}
pub fn parse_subquery(p: &mut Parser) -> Result<Option<SubQuery>, DMLParseError> {
    let token = p.peek_token();
    match token {
        Some(Token::LParen) => {
            let _ = p.token_next();
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
    Ok(QueryBlock {
        with: None,
        distinct,
        select_list,
        from: None,
    })
    // Err(DMLParseError::SelectParseError("jopa".to_string()))
}

fn parse_select_list(p: &mut Parser) -> Result<(Distinc, Vec<SelectItem>), DMLParseError> {
    let mut distinct = Distinc::All;
    let mut select_list = vec![];
    let mut list_token = p.next_token().unwrap_or_else(|| &Token::EOF);
    loop {
        // print!("Token<{:?}>", list_token);
        match list_token {
            Token::Keyword(Keyword::From) => break,
            Token::LParen => break,
            Token::Comma => {}
            Token::Keyword(Keyword::Distinct) | Token::Keyword(Keyword::Unique) => {
                distinct = Distinc::Distinct;
            }
            Token::Asterisk(_) => {
                let expression = SimpleExpression {
                    table: None,
                    schema: None,
                    column: "*".to_string(),
                };

                select_list.push(SelectItem {
                    expression: Expression::SimpleExpression(expression),
                    alias: None,
                })
            }
            // trying to parse select list.
            // See the schema: https://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/SELECT.html#GUID-CFA006CA-6FF1-4972-821E-6996142A51C6
            // Variants:
            // 1. schema.table.column as alias
            // 2. schema.table.column alias
            // 3. schema.table.column
            // 4. schema.table.*

            // 5. table.column as alias
            // 6. table.column alias
            // 7. table.column
            // 8. table.*

            // 9. column as alias
            // 10. column alias
            // 11. column

            // TODO: refactor this s..
            Token::Identifier(_) => {
                let (t1, t2, t3, t4, t5, t6, t7, t8): (
                    Option<&Token>,
                    Option<&Token>,
                    Option<&Token>,
                    Option<&Token>,
                    Option<&Token>,
                    Option<&Token>,
                    Option<&Token>,
                    Option<&Token>,
                ) = (
                    p.peek_nth(0),
                    p.peek_nth(1),
                    p.peek_nth(2),
                    p.peek_nth(3),
                    p.peek_nth(4),
                    p.peek_nth(5),
                    p.peek_nth(6),
                    p.peek_nth(7),
                );
                match (t1, t2, t3, t4, t5, t6, t7, t8) {
                    // 1. schema.table.column as alias
                    (
                        Some(Token::Identifier(schema)),
                        Some(Token::Dot),
                        Some(Token::Identifier(table)),
                        Some(Token::Dot),
                        Some(Token::Identifier(column)),
                        Some(Token::Keyword(Keyword::As)),
                        Some(Token::Identifier(alias)),
                        Some(Token::Comma) | Some(Token::Keyword(Keyword::From)),
                    ) => {
                        let expression = SimpleExpression {
                            schema: Some(schema.to_string()),
                            table: Some(table.to_string()),
                            column: column.to_string(),
                        };
                        select_list.push(SelectItem {
                            expression: Expression::SimpleExpression(expression),
                            alias: Some(alias.to_string()),
                        });
                        println!("1: {:?} {:?}", t1, t2);
                        let idx = p.move_index(7);
                        println!("1: {:?}", idx);
                    }
                    // 2. schema.table.column alias
                    (
                        Some(Token::Identifier(schema)),
                        Some(Token::Dot),
                        Some(Token::Identifier(table)),
                        Some(Token::Dot),
                        Some(Token::Identifier(column)),
                        Some(Token::Identifier(alias)),
                        Some(Token::Comma) | Some(Token::Keyword(Keyword::From)),
                        _,
                    ) => {
                        let expression = SimpleExpression {
                            schema: Some(schema.to_string()),
                            table: Some(table.to_string()),
                            column: column.to_string(), // Поскольку это Asterisk
                        };
                        select_list.push(SelectItem {
                            expression: Expression::SimpleExpression(expression),
                            alias: Some(alias.to_string()),
                        });
                        let _ = p.move_index(6);
                    }
                    // 3. schema.table.column
                    (
                        Some(Token::Identifier(schema)),
                        Some(Token::Dot),
                        Some(Token::Identifier(table)),
                        Some(Token::Dot),
                        Some(Token::Identifier(column)),
                        Some(Token::Comma) | Some(Token::Keyword(Keyword::From)),
                        _,
                        _,
                    ) => {
                        let expression = SimpleExpression {
                            schema: Some(schema.to_string()),
                            table: Some(table.to_string()),
                            column: column.to_string(), // Поскольку это Asterisk
                        };
                        select_list.push(SelectItem {
                            expression: Expression::SimpleExpression(expression),
                            alias: None,
                        });
                        let _ = p.move_index(5);
                    }
                    // 4. schema.table.*
                    (
                        Some(Token::Identifier(schema)),
                        Some(Token::Dot),
                        Some(Token::Identifier(table)),
                        Some(Token::Dot),
                        Some(Token::Asterisk('*')),
                        Some(Token::Comma) | Some(Token::Keyword(Keyword::From)),
                        _,
                        _,
                    ) => {
                        let expression = SimpleExpression {
                            schema: Some(schema.to_string()),
                            table: Some(table.to_string()),
                            column: "*".to_string(), // Поскольку это Asterisk
                        };
                        select_list.push(SelectItem {
                            expression: Expression::SimpleExpression(expression),
                            alias: None,
                        });
                        let _ = p.move_index(4);
                    }
                    // 5. table.column as alias
                    (
                        Some(Token::Identifier(table)),
                        Some(Token::Dot),
                        Some(Token::Identifier(column)),
                        Some(Token::Keyword(Keyword::As)),
                        Some(Token::Identifier(alias)),
                        Some(Token::Comma) | Some(Token::Keyword(Keyword::From)),
                        _,
                        _,
                    ) => {
                        let expression = SimpleExpression {
                            schema: None,
                            table: Some(table.to_string()),
                            column: column.to_string(), // Поскольку это Asterisk
                        };
                        select_list.push(SelectItem {
                            expression: Expression::SimpleExpression(expression),
                            alias: Some(alias.to_string()),
                        });
                        let _ = p.move_index(4);
                    }
                    // 6. table.column alias
                    (
                        Some(Token::Identifier(table)),
                        Some(Token::Dot),
                        Some(Token::Identifier(column)),
                        Some(Token::Identifier(alias)),
                        Some(Token::Comma) | Some(Token::Keyword(Keyword::From)),
                        _,
                        _,
                        _,
                    ) => {
                        let expression = SimpleExpression {
                            schema: None,
                            table: Some(table.to_string()),
                            column: column.to_string(), // Поскольку это Asterisk
                        };
                        select_list.push(SelectItem {
                            expression: Expression::SimpleExpression(expression),
                            alias: None,
                        });
                        let _ = p.move_index(3);
                    }
                    // 7. table.column
                    (
                        Some(Token::Identifier(table)),
                        Some(Token::Dot),
                        Some(Token::Identifier(column)),
                        Some(Token::Comma) | Some(Token::Keyword(Keyword::From)),
                        _,
                        _,
                        _,
                        _,
                    ) => {
                        let expression = SimpleExpression {
                            schema: None,
                            table: Some(table.to_string()),
                            column: column.to_string(),
                        };
                        select_list.push(SelectItem {
                            expression: Expression::SimpleExpression(expression),
                            alias: None,
                        });
                        let _ = p.move_index(2);
                    }
                    // 8. table.*
                    (
                        Some(Token::Identifier(table)),
                        Some(Token::Dot),
                        Some(Token::Asterisk('*')),
                        Some(Token::Comma) | Some(Token::Keyword(Keyword::From)),
                        _,
                        _,
                        _,
                        _,
                    ) => {
                        let expression = SimpleExpression {
                            schema: None,
                            table: Some(table.to_string()),
                            column: "*".to_string(), // Поскольку это Asterisk
                        };
                        select_list.push(SelectItem {
                            expression: Expression::SimpleExpression(expression),
                            alias: None,
                        });
                        let _ = p.move_index(2);
                    }
                    // 9. column as alias
                    (
                        Some(Token::Identifier(column)),
                        Some(Token::Keyword(Keyword::As)),
                        Some(Token::Identifier(alias)),
                        Some(Token::Comma) | Some(Token::Keyword(Keyword::From)),
                        _,
                        _,
                        _,
                        _,
                    ) => {
                        let expression = SimpleExpression {
                            schema: None,
                            table: None,
                            column: column.to_string(), // Поскольку это Asterisk
                        };
                        select_list.push(SelectItem {
                            expression: Expression::SimpleExpression(expression),
                            alias: Some(alias.to_string()),
                        });
                        let _ = p.move_index(3);
                    }
                    // 10. column alias
                    (
                        Some(Token::Identifier(column)),
                        Some(Token::Identifier(alias)),
                        Some(Token::Comma) | Some(Token::Keyword(Keyword::From)),
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => {
                        let expression = SimpleExpression {
                            schema: None,
                            table: None,
                            column: column.to_string(), // Поскольку это Asterisk
                        };
                        select_list.push(SelectItem {
                            expression: Expression::SimpleExpression(expression),
                            alias: Some(alias.to_string()),
                        });
                        let _ = p.move_index(2);
                    }
                    // 11. column
                    (
                        Some(Token::Identifier(column)),
                        Some(Token::Comma) | Some(Token::Keyword(Keyword::From)),
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => {
                        let expression = SimpleExpression {
                            schema: None,
                            table: None,
                            column: column.to_string(), // Поскольку это Asterisk
                        };
                        select_list.push(SelectItem {
                            expression: Expression::SimpleExpression(expression),
                            alias: None,
                        });
                        let _ = p.move_index(1);
                    }
                    _ => {}
                }
            }
            _ => break,
        }
        list_token = p.next_token().unwrap_or_else(|| &Token::EOF);
        println!("{:?}", list_token);
    }

    Ok((distinct, select_list))
}

// fn parse_select_item(p: &mut Parser) -> Result<ColumnAccess, DMLParseError> {
//     Err(DMLParseError::SelectParseError(
//         "Not implemented".to_string(),
//     ))
// }
