mod errors;
mod reader;
mod sqlite;

use crate::errors::*;

pub use sqlite::*;

pub trait Operation {
    fn execute(&mut self) -> Result<(), EngineError> {
        todo!()
    }
}
