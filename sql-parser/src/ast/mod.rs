mod entities;
mod expression;

pub use entities::{CreateTable, Delete, DropTable, Insert, Ordering, Select, Update};
pub use expression::{ColumnLiteral, Expression, Literal};

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
