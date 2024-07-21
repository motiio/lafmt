use std::str::FromStr;

use derive_more::Display;

#[derive(Debug, Display, PartialEq, Clone)]
pub enum Keyword {
    Select,
    From,
}

pub struct ParseKeywordError;

impl FromStr for Keyword {
    type Err = ParseKeywordError;
    fn from_str(s: &str) -> Result<Keyword, ParseKeywordError> {
        match s.trim().to_lowercase().as_str() {
            "select" => Ok(Keyword::Select),
            "from" => Ok(Keyword::From),
            _ => Err(ParseKeywordError),
        }
    }
}
