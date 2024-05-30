mod catalog;
mod file_format;
mod header;
mod page;
mod schema_format;
mod text_encoding;

pub use catalog::*;
pub use file_format::*;
pub use header::*;
pub use page::*;
pub use schema_format::*;
pub use text_encoding::*;

use crate::errors::*;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
pub struct Database {
    pub header: Header,
    pub catalog: Catalog,
}

impl Database {
    pub fn load(data: Vec<u8>) -> Result<Self, DatabaseError> {
        let header = Header::load(&data[0..=100])?;

        let catalog = Catalog::load(&data[100..header.page_size as usize])?;

        Ok(Self { header, catalog })
    }

    pub fn open<P: AsRef<Path>>(file: P) -> Result<Self, DatabaseError> {
        let mut file =
            File::open(file).map_err(|e| DatabaseError::FileSystemError(format!("{e:?}")))?;
        let metadata = file
            .metadata()
            .map_err(|e| DatabaseError::FileSystemError(format!("{e:?}")))?;
        let mut data = vec![0; metadata.len() as usize];
        file.read_exact(&mut data)
            .map_err(|e| DatabaseError::FileSystemError(format!("{e:?}")))?;
        Database::load(data)
    }

    pub fn execute_sql(&mut self, _query: String) -> Result<(), DatabaseError> {
        println!("{:?}", &self);
        todo!()
    }

    pub fn print_info(&self) {
        let header = &self.header;

        println!("{:<20} {}", "database page size:", self.header.page_size);
        println!("{:<20} {}", "write format:", header.write_format as u8);
        println!("{:<20} {}", "read format:", header.read_format as u8);
        println!("{:<20} {}", "reserved bytes:", header.reserved_bytes);
        println!(
            "{:<20} {}",
            "file change counter:", header.file_change_counter
        );
        println!(
            "{:<20} {}",
            "database page count:", header.database_page_count
        );
        println!(
            "{:<20} {}",
            "freelist page count:", header.freelist_page_count
        );
        println!("{:<20} {}", "schema cookie:", header.schema_cookie);
        println!("{:<20} {}", "schema format:", header.schema_format as u8);
        println!(
            "{:<20} {}",
            "default cache size:", header.default_page_cache_size
        );
        println!(
            "{:<20} {}",
            "autovacuum top root:", header.autovacuum_top_root
        );
        println!(
            "{:<20} {}",
            "incremental vacuum:", header.incremental_vacuum_mode as u8
        );
        println!("{:<20} {}", "text encoding:", header.text_encoding);
        println!("{:<20} {}", "user version:", header.user_version);
        println!("{:<20} {}", "application id:", header.application_id);
        println!("{:<20} {}", "software version:", header.sqlite_version);
        println!("{:<20} {}", "number of tables:", self.catalog.tables_count);
        // println!("number of indexes: {:>21}", todo!())?;
        // println!("number of triggers: {:>21}", todo!())?;
        // println!("number of views: {:>21}", todo!())?;
        // println!("schema size: {:>21}", todo!())?;
        // println!("data version: {:>21}", todo!())
    }
}
