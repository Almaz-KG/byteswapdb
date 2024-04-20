use crate::ast::{Expresion, Ordering};
use common::types::Column;

#[derive(Debug, PartialEq)]
pub struct Select {
    pub columns: Vec<Expresion>,
    pub from: String,
    pub where_clause: Option<Expresion>,
    pub group_by: Option<Vec<Column>>,
    pub having: Option<Expresion>,
    pub order_by: Option<Vec<(Column, Ordering)>>,
    pub limit: Option<usize>,
    pub distinct: bool,
}

#[derive(Debug, PartialEq)]
pub struct CreateTable {
    pub table_name: String,
    pub columns: Vec<Column>,
}

#[derive(Debug, PartialEq)]
pub struct DropTable {
    pub table_name: String,
    pub if_exists: bool,
}

#[derive(Debug, PartialEq)]
pub struct Delete {
    pub table_name: String,
    pub where_clause: Option<Expresion>,
}

#[derive(Debug, PartialEq)]
pub struct Insert {
    pub table_name: String,
    pub columns: Vec<Column>,
    pub values: Vec<Vec<Expresion>>,
}

#[derive(Debug, PartialEq)]
pub struct Update {
    pub table_name: String,
    pub set_clause: Vec<(Column, Expresion)>,
    pub where_clause: Option<Expresion>,
}
