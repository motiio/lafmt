
pub struct Parser<'a> {
    pub raw_code: &'a String,
}

impl Parser<'_> {
    pub fn new(raw_code: &String) -> Parser {
        Parser { raw_code }
    }
}
