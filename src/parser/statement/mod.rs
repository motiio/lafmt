mod dml;
pub use dml::parse_dml_statement;
pub use dml::DMLParseError;

pub enum Statement{
    // SELECT
    Query(Box<Query>)
}

pub struct Query {
    pub body: Box<QueryBody>,
}

pub enum QueryBody {
    Select(Box<dml::Select>),
}


