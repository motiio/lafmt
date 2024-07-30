use super::Ident;

#[derive(Debug)]
pub struct ColumnAccess {
    pub column: Ident,
    pub alias: Ident,
}
