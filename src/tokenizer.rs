use regex::Regex;

use derive_more::Display;

#[derive(Debug, Display, PartialEq)]
pub enum Token<'a> {
    Keyword(&'a str),
    Identifier(&'a str),
    Delimetr(&'a str),
    NumberLiteral(f64),
    MathOperator(&'a str),
    StringLiteral(&'a str),
    Asterisk(&'a str),
    Unknown(&'a str),
}
pub struct Tokenizer<'a> {
    raw_code: &'a str,
    _tokens: Vec<Token<'a>>,
}

impl Tokenizer<'_> {
    pub fn new(raw_code: &str) -> Tokenizer {
        Tokenizer {
            raw_code,
            _tokens: Vec::new(),
        }
    }

    pub fn produce_tokens(&mut self) -> &Vec<Token> {
        let re = Regex::new(
            r#"(?x)
              (?P<keyword>\b(?i)(with|select|as|from|left|right|inner|join|on|where|and|or|insert|into|values|update|set|delete|create|table|drop|order\s+by|group\s+by)\b)
            | (?P<identifier>[a-zA-Z_][a-zA-Z0-9_]*)
            | (?P<number>\d+(\.\d+)?)
            | (?P<operator>[+\-/\*])
            | (?P<string>\'(?:\'\'|[^\'])*\'|\"(?:\"\"|[^\"])*\") 
            | (?P<delimetr>[,;().])
            | (?P<unknown>\S+)"#
        ).unwrap();

        for cap in re.captures_iter(self.raw_code) {
            if let Some(keyword) = cap.name("keyword") {
                self._tokens.push(Token::Keyword(keyword.as_str()));
            } else if let Some(identifier) = cap.name("identifier") {
                self._tokens.push(Token::Identifier(identifier.as_str()));
            } else if let Some(number) = cap.name("number") {
                self._tokens
                    .push(Token::NumberLiteral(number.as_str().parse().unwrap()));
            } else if let Some(asterisk) = cap.name("asterisk") {
                self._tokens.push(Token::Asterisk(asterisk.as_str()));
            } else if let Some(operator) = cap.name("operator") {
                // Если мы распарсили токен "*", то нужно проверить является ли он астериском,
                // иначе он математическое выражение
                if operator.as_str() == "*" {
                    if let Some(last_token) = self._tokens.last() {
                        match last_token {
                            // Если перед опетарором "*" стоит select или ".", то рассцениваем его
                            // как астериск.
                            // Пример: select * from dula; или select kek.* from kek;
                            Token::Delimetr(".") | Token::Keyword("select") => {
                                self._tokens.push(Token::Asterisk(operator.as_str()));
                                continue; // Пропускаем добавление MathOperator
                            }
                            _ => {}
                        }
                    }
                }
                self._tokens.push(Token::MathOperator(operator.as_str()));
            } else if let Some(string) = cap.name("string") {
                self._tokens.push(Token::StringLiteral(string.as_str()));
            } else if let Some(delimetr) = cap.name("delimetr") {
                self._tokens.push(Token::Delimetr(delimetr.as_str()));
            } else if let Some(unknown) = cap.name("unknown") {
                self._tokens.push(Token::Unknown(unknown.as_str()));
            }
        }

        &self._tokens
    }

    pub fn tokens(self: &Self) -> &Vec<Token> {
        &self._tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_literal() {
        let code = r#""kek""#;
        let mut tokenizer = Tokenizer::new(code);
        let tokens = tokenizer.produce_tokens();

        let expected_tokens = vec![Token::StringLiteral("\"kek\"")];

        assert_eq!(*tokens, expected_tokens);
    }

    #[test]
    fn test_num_literal_int() {
        let code = r#"1"#;
        let mut tokenizer = Tokenizer::new(code);
        let tokens = tokenizer.produce_tokens();

        let expected_tokens = vec![Token::NumberLiteral(f64::from(1))];

        assert_eq!(*tokens, expected_tokens);
    }

    #[test]
    fn test_num_literal_float() {
        let code = r#"123.321"#;
        let mut tokenizer = Tokenizer::new(code);
        let tokens = tokenizer.produce_tokens();

        let expected_tokens = vec![Token::NumberLiteral(123.321)];

        assert_eq!(*tokens, expected_tokens);
    }

    #[test]
    fn test_asterisk_1() {
        let code = r#"select * from dual;"#;
        let mut tokenizer = Tokenizer::new(code);
        let tokens = tokenizer.produce_tokens();

        let expected_tokens = vec![
            Token::Keyword("select"),
            Token::Asterisk("*"),
            Token::Keyword("from"),
            Token::Identifier("dual"),
            Token::Delimetr(";"),
        ];

        assert_eq!(*tokens, expected_tokens);
    }

    #[test]
    fn test_asterisk_2() {
        let code = r#"select kek.* from dual;"#;
        let mut tokenizer = Tokenizer::new(code);
        let tokens = tokenizer.produce_tokens();

        let expected_tokens = vec![
            Token::Keyword("select"),
            Token::Identifier("kek"),
            Token::Delimetr("."),
            Token::Asterisk("*"),
            Token::Keyword("from"),
            Token::Identifier("dual"),
            Token::Delimetr(";"),
        ];

        assert_eq!(*tokens, expected_tokens);
    }
}
