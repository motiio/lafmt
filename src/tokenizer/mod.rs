pub mod keyword;
mod string_buf;

use derive_more::Display;

use self::keyword::Keyword;
use self::string_buf::{StringBuf, StringBufIterator};

#[derive(Display, Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(Keyword),

    Identifier(String),

    NumberLiteral(String),
    StringLiteral(String),

    Asterisk(char),

    Eq,

    Dot,
    Comma,

    Colon,
    Semicolon,

    EOF,
}

pub fn tokenize(query: &str) -> Result<Vec<Token>, String> {
    let mut tokens = vec![];
    let buff = StringBuf::new(query);
    let mut buff_iter = buff.iter();

    while let Some(ch) = buff_iter.next() {
        let token = match ch {
            _ if ch.is_whitespace() => continue,
            'A'..='Z' | 'a'..='z' => tokenize_string(&mut buff_iter)?,
            '0'..='9' => tokenize_number(&mut buff_iter)?,
            '\"' | '\'' => tokenize_quoted_literal(&mut buff_iter)?,
            '*' => Token::Asterisk('*'),
            ':' => Token::Colon,
            ';' => Token::Semicolon,
            '.' => Token::Dot,
            ',' => Token::Comma,
            '=' => Token::Eq,
            _ => return Err(format!("Unexpected token '{}'", ch)),
        };
        tokens.push(token)
    }
    tokens.push(Token::EOF);

    Ok(tokens)
}

fn tokenize_string(buff_iter: &mut StringBufIterator) -> Result<Token, String> {
    buff_iter.prev();
    let word = buff_iter.fetch_while(|ch| matches!(ch, 'A'..='Z' | 'a'..='z' | '0'..='9' | '_' ));

    if let Ok(keyword) = word.parse::<Keyword>() {
        return Ok(Token::Keyword(keyword));
    }
    if let Ok(identifier) = word.parse::<String>() {
        return Ok(Token::Identifier(identifier));
    }
    Err(format!("Unexpected string token {}", word))
}

fn tokenize_number(buff_iter: &mut StringBufIterator) -> Result<Token, String> {
    buff_iter.prev();
    let word = buff_iter.fetch_while(|ch| matches!(ch, '0'..='9' | '.'));

    match word.parse::<f64>() {
        Ok(w) => Ok(Token::NumberLiteral(w.to_string())),
        Err(_) => Err(format!("Unexpected number token {}", word)),
    }
}

fn tokenize_quoted_literal(buff_iter: &mut StringBufIterator) -> Result<Token, String> {
    let mut string_literal = String::new();

    match buff_iter.prev() {
        Some('\"') => {
            buff_iter.next();
            if let Some(word) = buff_iter.fetch_to_delim("\"") {
                string_literal.push_str(word)
            }
            Ok(Token::Identifier(string_literal))
        }
        Some('\'') => {
            buff_iter.next();
            if let Some(word) = buff_iter.fetch_to_delim("\'") {
                string_literal.push_str(word)
            }
            Ok(Token::StringLiteral(string_literal))
        }

        _ => Err("Oops".to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::tokenizer::keyword::Keyword;
    use crate::tokenizer::{tokenize, Token};

    #[test]
    pub fn test_simple_query() {
        let query = "select * from kek;\n";

        if let Ok(tokens) = tokenize(&query) {
            assert_eq!(
                vec![
                    Token::Keyword(Keyword::Select),
                    Token::Asterisk('*'),
                    Token::Keyword(Keyword::From),
                    Token::Identifier(String::from("kek")),
                    Token::Semicolon,
                ],
                tokens
            );
        } else {
            assert_eq!(1, 0);
        }
    }

    #[test]
    pub fn test_complex_identifier() {
        let query = "select maf_123 from kek;\n";

        if let Ok(tokens) = tokenize(&query) {
            assert_eq!(
                vec![
                    Token::Keyword(Keyword::Select),
                    Token::Identifier(String::from("maf_123")),
                    Token::Keyword(Keyword::From),
                    Token::Identifier(String::from("kek")),
                    Token::Semicolon,
                ],
                tokens
            );
        } else {
            assert_eq!(1, 0);
        }
    }

    #[test]
    pub fn test_alias_query() {
        let query = "select mem as lol from kek;\n";

        if let Ok(tokens) = tokenize(&query) {
            assert_eq!(
                vec![
                    Token::Keyword(Keyword::Select),
                    Token::Identifier(String::from("mem")),
                    Token::Keyword(Keyword::As),
                    Token::Identifier(String::from("lol")),
                    Token::Keyword(Keyword::From),
                    Token::Identifier(String::from("kek")),
                    Token::Semicolon,
                ],
                tokens
            );
        } else {
            assert_eq!(1, 0);
        }
    }

    #[test]
    pub fn test_f64_num_query() {
        let query = "select 123.321 lol from kek;\n";

        if let Ok(tokens) = tokenize(&query) {
            assert_eq!(
                vec![
                    Token::Keyword(Keyword::Select),
                    Token::NumberLiteral("123.321".to_string()),
                    Token::Identifier(String::from("lol")),
                    Token::Keyword(Keyword::From),
                    Token::Identifier(String::from("kek")),
                    Token::Semicolon,
                ],
                tokens
            );
        } else {
            assert_eq!(1, 0);
        }
    }

    #[test]
    pub fn test_i32_num_query() {
        let query = "select 123 lol from kek;\n";

        if let Ok(tokens) = tokenize(&query) {
            assert_eq!(
                vec![
                    Token::Keyword(Keyword::Select),
                    Token::NumberLiteral("123".to_string()),
                    Token::Identifier(String::from("lol")),
                    Token::Keyword(Keyword::From),
                    Token::Identifier(String::from("kek")),
                    Token::Semicolon,
                ],
                tokens
            );
        } else {
            assert_eq!(1, 0);
        }
    }
    #[test]
    pub fn test_join_query() {
        let query = "select 123, mem from kek join lol on kek.id=lol.id;\n";

        if let Ok(tokens) = tokenize(&query) {
            assert_eq!(
                vec![
                    Token::Keyword(Keyword::Select),
                    Token::NumberLiteral("123".to_string()),
                    Token::Comma,
                    Token::Identifier("mem".to_string()),
                    Token::Keyword(Keyword::From),
                    Token::Identifier(String::from("kek")),
                    Token::Keyword(Keyword::Join),
                    Token::Identifier(String::from("lol")),
                    Token::Keyword(Keyword::On),
                    Token::Identifier(String::from("kek")),
                    Token::Dot,
                    Token::Identifier(String::from("id")),
                    Token::Eq,
                    Token::Identifier(String::from("lol")),
                    Token::Dot,
                    Token::Identifier(String::from("id")),
                    Token::Semicolon,
                ],
                tokens
            );
        } else {
            assert_eq!(1, 0);
        }
    }

    #[test]
    pub fn test_string_literal_query() {
        let query = "select \'123\', \'mem\' from kek;\n";

        if let Ok(tokens) = tokenize(&query) {
            assert_eq!(
                vec![
                    Token::Keyword(Keyword::Select),
                    Token::StringLiteral("123".to_string()),
                    Token::Comma,
                    Token::StringLiteral(String::from("mem")),
                    Token::Keyword(Keyword::From),
                    Token::Identifier(String::from("kek")),
                    Token::Semicolon,
                ],
                tokens
            );
        } else {
            assert_eq!(1, 0);
        }
    }

    #[test]
    pub fn test_quoted_identifier_query() {
        let query = "select \'123\', \"mem\" from kek;\n";

        if let Ok(tokens) = tokenize(&query) {
            assert_eq!(
                vec![
                    Token::Keyword(Keyword::Select),
                    Token::StringLiteral("123".to_string()),
                    Token::Comma,
                    Token::Identifier(String::from("mem")),
                    Token::Keyword(Keyword::From),
                    Token::Identifier(String::from("kek")),
                    Token::Semicolon,
                ],
                tokens
            );
        } else {
            assert_eq!(1, 0);
        }
    }
}
