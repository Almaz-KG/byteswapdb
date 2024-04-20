mod expression;

use common::types::Column;

pub use expression::{Expresion, Literal};

#[derive(Debug, PartialEq)]
pub enum Ordering {
    Ascending,
    Descending,
}

#[derive(Debug, PartialEq)]
pub enum Ast {
    Select {
        columns: Vec<Expresion>,
        from: String,
        where_clause: Option<Expresion>,
        group_by: Option<Vec<Column>>,
        having: Option<Expresion>,
        order_by: Option<Vec<(Column, Ordering)>>,
        limit: Option<usize>,
        distinct: bool,
    },
    CreateTable {
        table_name: String,
        columns: Vec<Column>,
    },
    DropTable {
        table_name: String,
        if_exists: bool,
    },
    Delete {
        table_name: String,
        where_clause: Option<Expresion>,
    },
    Insert {
        table_name: String,
        columns: Vec<Column>,
        values: Vec<Vec<Expresion>>,
    },
    Update {
        table_name: String,
        set_clause: Vec<(Column, Expresion)>,
        where_clause: Option<Expresion>,
    },
    Explain(Box<Ast>),
}
