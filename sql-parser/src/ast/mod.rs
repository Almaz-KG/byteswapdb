mod entities;
mod expression;

pub use entities::{CreateTable, Delete, DropTable, Insert, Select, Update, Ordering};
pub use expression::{Expression, Literal, ColumnLiteral};

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
