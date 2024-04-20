mod entities;
mod expression;

pub use entities::{CreateTable, Delete, DropTable, Insert, Select, Update};
pub use expression::{Expresion, Literal};

#[derive(Debug, PartialEq)]
pub enum Ordering {
    Ascending,
    Descending,
}

#[derive(Debug, PartialEq)]
pub enum Ast {
    Select(Select),
    CreateTable(CreateTable),
    DropTable(DropTable),
    Delete(Delete),
    Insert(Insert),
    Update(Update),
    Explain(Box<Ast>),
}
