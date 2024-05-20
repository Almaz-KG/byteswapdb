pub mod database;

pub use database::*;

use crate::errors::*;
use std::fs::File;
use std::path::Path;
use std::io::Read;

pub struct SQLiteEngine {
    database: SQLiteDatabase,
}

impl SQLiteEngine {

    pub fn open<P: AsRef<Path>>(file: P) -> Result<Self, EngineError> {
        let mut file = File::open(file).map_err(|e| EngineError::FileSystemError(format!("{e:?}")))?;
        let metadata = file.metadata().map_err(|e| EngineError::FileSystemError(format!("{e:?}")))?;
        let mut data = vec![0; metadata.len() as usize];
        file.read_exact(&mut data).map_err(|e| EngineError::FileSystemError(format!("{e:?}")))?;
        
        let database = SQLiteDatabase::load(data)?;
        dbg!(&database);
        Ok(Self{database})
    }

    pub fn execute_sql(&mut self, _query: String) -> Result<(), EngineError> {
        println!("{:?}", &self.database);
        todo!()
    }
    
}
