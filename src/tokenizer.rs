use derive_more::Display;

use crate::keyword::Keyword;
use crate::string_buf::{StringBuf, StringBufIterator};

#[derive(Display, Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword),

    Identifier(String),

    NumberLiteral(f64),
    StringLiteral(String),

    Asterisk(char),

    Equal,

    Dot,
    Comma,

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
            ';' => Token::Semicolon,
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
        Ok(w) => Ok(Token::NumberLiteral(w)),
        Err(_) => Err(format!("Unexpected number token {}", word)),
    }
}


