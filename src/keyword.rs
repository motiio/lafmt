use std::str::FromStr;

use derive_more::Display;

#[derive(Debug, Display, PartialEq, Clone)]
pub enum Keyword {
    With,
    Select,
    From,
    As,
    Left,
    Right,
    Join,
    On,
}

pub struct ParseKeywordError;

impl FromStr for Keyword {
    type Err = ParseKeywordError;
    fn from_str(s: &str) -> Result<Keyword, ParseKeywordError> {
        match s.trim().to_lowercase().as_str() {
            "with" => Ok(Keyword::With),
            "select" => Ok(Keyword::Select),
            "from" => Ok(Keyword::From),
            "as" => Ok(Keyword::As),
            "left" => Ok(Keyword::Left),
            "right" => Ok(Keyword::Right),
            "join" => Ok(Keyword::Join),
            "on" => Ok(Keyword::On),
            _ => Err(ParseKeywordError),
        }
    }
}
