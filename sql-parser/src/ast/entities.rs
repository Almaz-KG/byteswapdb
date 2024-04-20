use crate::ast::{Expression, ColumnLiteral};

#[derive(Debug, PartialEq)]
pub struct Select {
    pub columns: Vec<ColumnLiteral>,
    pub from: String,
    pub where_clause: Option<Expression>,
    pub group_by: Option<Vec<String>>,
    pub having: Option<Expression>,
    pub order_by: Option<Vec<(String, Ordering)>>,
    pub limit: Option<usize>,
    pub distinct: bool,
}

#[derive(Debug, PartialEq)]
pub struct CreateTable {
    pub table_name: String,
    pub columns: Vec<ColumnLiteral>,
}

#[derive(Debug, PartialEq)]
pub struct DropTable {
    pub table_name: String,
    pub if_exists: bool,
}

#[derive(Debug, PartialEq)]
pub struct Delete {
    pub table_name: String,
    pub where_clause: Option<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct Insert {
    pub table_name: String,
    pub columns: Vec<String>,
    pub values: Vec<Vec<Expression>>,
}

#[derive(Debug, PartialEq)]
pub struct Update {
    pub table_name: String,
    pub set_clause: Vec<(String, Expression)>,
    pub where_clause: Option<Expression>,
}


#[derive(Debug, PartialEq)]
pub enum Ordering {
    Ascending,
    Descending,
}