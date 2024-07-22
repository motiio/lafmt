use derive_more::Display;

use crate::keyword::Keyword;
use crate::string_buf::{StringBuf, StringBufIterator};

#[derive(Display, Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword),

    Identifier(String),

    NumberLiteral(String),
    StringLiteral(String),

    Asterisk(char),

    Equal,

    Dot,
    Comma,

    Colon,
    Semicolon,
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
            '*' => Token::Asterisk('*'),
            ':' => Token::Colon,
            ';' => Token::Semicolon,
            '.' => Token::Dot,
            ',' => Token::Comma,
            '=' => Token::Equal,
            _ => return Err(format!("Unexpected token '{}'", ch)),
        };
        tokens.push(token)
    }

    Ok(tokens)
}

fn tokenize_string<'a>(buff_iter: &mut StringBufIterator) -> Result<Token, String> {
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

fn tokenize_number<'a>(buff_iter: &mut StringBufIterator) -> Result<Token, String> {
    buff_iter.prev();
    let word = buff_iter.fetch_while(|ch| matches!(ch, '0'..='9' | '.'));

    match word.parse::<f64>() {
        Ok(w) => Ok(Token::NumberLiteral(w.to_string())),
        Err(_) => Err(format!("Unexpected number token {}", word)),
    }
}

#[cfg(test)]
mod test {

    use crate::keyword::Keyword;
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
}
