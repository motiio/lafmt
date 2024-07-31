mod dml;
pub use dml::parse_dml_statement;
pub use dml::DMLParseError;

# [derive(Debug)]
pub enum Statement{
    // SELECT
    Query(Box<Query>)
}

# [derive(Debug)]
pub struct Query {
    pub body: Box<QueryBody>,
}

# [derive(Debug)]
pub enum QueryBody {
    Select(Box<dml::Select>),
}


