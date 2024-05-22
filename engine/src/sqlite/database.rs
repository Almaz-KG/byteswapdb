use std::fmt::Display;

use crate::errors::*;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum FileFormat {
    Legacy = 1,
    WAL = 2,
}

impl TryFrom<u8> for FileFormat {
    type Error = EngineError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(FileFormat::Legacy),
            2 => Ok(FileFormat::WAL),
            _ => Err(EngineError::StateError(format!("Unknown file format: {value}")))
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum TextEncoding {
    UTF_8 = 1,
    UTF_16le = 2,
    UTF_16be = 3,
}

impl TryFrom<u32> for TextEncoding {
    type Error = EngineError;

    /// The database text encoding. A value of 1 means UTF-8. A value of 2 means UTF-16le. A value of 3 means UTF-16be.
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(TextEncoding::UTF_8),
            2 => Ok(TextEncoding::UTF_16le),
            3 => Ok(TextEncoding::UTF_16be),
            _ => Err(EngineError::StateError(format!("Unsupported text encoding: {value}")))
        }
    }
}

impl Display for TextEncoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextEncoding::UTF_8 => write!(f, "1 (utf8)"),
            TextEncoding::UTF_16le => write!(f, "2 (utf16le)"),
            TextEncoding::UTF_16be => write!(f, "3 (utf16be)"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum SchemaFormat {
    Format1 = 1,
    Format2 = 2,
    Format3 = 3,
    Format4 = 4,
}

impl TryFrom<u32> for SchemaFormat {
    type Error = EngineError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(SchemaFormat::Format1),
            2 => Ok(SchemaFormat::Format2),
            3 => Ok(SchemaFormat::Format3),
            4 => Ok(SchemaFormat::Format4),
            _ => Err(EngineError::StateError(format!("Unsupported schema format: {value}")))
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct SQLiteDatabaseHeader {
    // 0	16	The header string: "SQLite format 3\000"
    pub header: String,
    // 16	2	The database page size in bytes. Must be a power of two between 512 and 32768 inclusive, or the value 1 representing a page size of 65536.
    page_size: u32,
    // 18	1	File format write version. 1 for legacy; 2 for WAL.
    write_format: FileFormat,
    // 19	1	File format read version. 1 for legacy; 2 for WAL.
    read_format: FileFormat,
    // 20	1	Bytes of unused "reserved" space at the end of each page. Usually 0.
    reserved_bytes: u8,
    // 21	1	Maximum embedded payload fraction. Must be 64.
    max_embedded_payload: u8,
    // 22	1	Minimum embedded payload fraction. Must be 32.
    min_embedded_payload: u8,
    // 23	1	Leaf payload fraction. Must be 32.
    lead_payload: u8,
    // 24	4	File change counter.
    file_change_counter: u32,
    // 28	4	Size of the database file in pages. The "in-header database size".
    database_page_count: u32,
    // 32	4	Page number of the first freelist trunk page.
    first_page_number_trunk_page: u32,
    // 36	4	Total number of freelist pages.
    freelist_page_count: u32,
    // 40	4	The schema cookie.
    schema_cookie: u32,
    // 44	4	The schema format number. Supported schema formats are 1, 2, 3, and 4.
    schema_format: SchemaFormat,
    // 48	4	Default page cache size.
    default_page_cache_size: u32,
    // 52	4	The page number of the largest root b-tree page when in auto-vacuum or incremental-vacuum modes, or zero otherwise.
    autovacuum_top_root: u32,
    // 56	4	The database text encoding. A value of 1 means UTF-8. A value of 2 means UTF-16le. A value of 3 means UTF-16be.
    text_encoding: TextEncoding,
    // 60	4	The "user version" as read and set by the user_version pragma.
    user_version: u32,
    // 64	4	True (non-zero) for incremental-vacuum mode. False (zero) otherwise.
    incremental_vacuum_mode: bool,
    // 68	4	The "Application ID" set by PRAGMA application_id.
    application_id: u32,
    // 72	20	Reserved for expansion. Must be zero.
    // reserved: Vec<u8>,
    // 92	4	The version-valid-for number.
    valid_for_verison: u32,
    // 96	4	SQLITE_VERSION_NUMBER
    sqlite_version: u32,
}

impl SQLiteDatabaseHeader {
    pub fn load(data: &[u8]) -> Result<Self, EngineError> {
        Ok(Self {
            header: String::from_utf8(data[0..16].to_vec()).map_err(|e| EngineError::StateError(format!("{e:?}")))?,
            page_size: SQLiteDatabaseHeader::load_page_size(&data[16..=17]),
            write_format: data[18].try_into()?,
            read_format: data[19].try_into()?,
            reserved_bytes: data[20],
            max_embedded_payload: data[21],
            min_embedded_payload: data[22],
            lead_payload: data[23],
            file_change_counter: u32::from_be_bytes([data[24], data[25], data[26], data[27]]),
            database_page_count: u32::from_be_bytes([data[28], data[29], data[30], data[31]]),
            first_page_number_trunk_page: u32::from_be_bytes([data[32], data[33], data[34], data[35]]),
            freelist_page_count: u32::from_be_bytes([data[36], data[37], data[38], data[39]]),
            schema_cookie: u32::from_be_bytes([data[40], data[41], data[42], data[43]]),
            schema_format: u32::from_be_bytes([data[44], data[45], data[46], data[47]]).try_into()?,
            default_page_cache_size: u32::from_be_bytes([data[48], data[49], data[50], data[51]]),
            autovacuum_top_root: u32::from_be_bytes([data[52], data[53], data[54], data[55]]),
            text_encoding: u32::from_be_bytes([data[56], data[57], data[58], data[59]]).try_into()?,
            user_version: u32::from_be_bytes([data[60], data[61], data[62], data[63]]),
            incremental_vacuum_mode: u32::from_be_bytes([data[64], data[65], data[66], data[67]]) != 0,
            application_id: u32::from_be_bytes([data[68], data[69], data[70], data[71]]),
            // reserved: data[72..92].to_vec(),
            valid_for_verison: u32::from_be_bytes([data[92], data[93], data[94], data[95]]),
            sqlite_version: u32::from_be_bytes([data[96], data[97], data[98], data[99]]),
        })
    }

    fn load_page_size(data: &[u8]) -> u32 {
        let x = u16::from_be_bytes([data[0], data[1]]);
        assert!((x & (x - 1)) == 0);
        if x == 1 {
            u16::MAX as u32 + 1
        } else {
            x as u32
        }
    }

}

#[derive(Debug)]
pub struct SQLiteDatabase {
    pub header: SQLiteDatabaseHeader
}

impl SQLiteDatabase {
    pub fn load(data: Vec<u8>) -> Result<Self, EngineError> {
        Ok(
            Self {
                header: SQLiteDatabaseHeader::load(&data[0..=100])?
            }
        )
    }
}

impl Display for SQLiteDatabase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let header = &self.header;
        
        writeln!(f, "{:<20} {}", "database page size:", self.header.page_size)?;
        writeln!(f, "{:<20} {}", "write format:", header.write_format as u8)?;
        writeln!(f, "{:<20} {}", "read format:", header.read_format as u8)?;
        writeln!(f, "{:<20} {}", "reserved bytes:", header.reserved_bytes)?;
        writeln!(f, "{:<20} {}", "file change counter:", header.file_change_counter)?;
        writeln!(f, "{:<20} {}", "database page count:", header.database_page_count)?;
        writeln!(f, "{:<20} {}", "freelist page count:", header.freelist_page_count)?;
        writeln!(f, "{:<20} {}", "schema cookie:", header.schema_cookie)?;
        writeln!(f, "{:<20} {}", "schema format:", header.schema_format as u8)?;
        writeln!(f, "{:<20} {}", "default cache size:", header.default_page_cache_size)?;
        writeln!(f, "{:<20} {}", "autovacuum top root:", header.autovacuum_top_root)?;
        writeln!(f, "{:<20} {}", "incremental vacuum:", header.incremental_vacuum_mode as u8)?;
        writeln!(f, "{:<20} {}", "text encoding:", header.text_encoding)?;
        writeln!(f, "{:<20} {}", "user version:", header.user_version)?;
        writeln!(f, "{:<20} {}", "application id:", header.application_id)?;
        writeln!(f, "{:<20} {}", "software version:", header.sqlite_version)
        // writeln!(f, "number of tables: {:>21}", todo!())?;
        // writeln!(f, "number of indexes: {:>21}", todo!())?;
        // writeln!(f, "number of triggers: {:>21}", todo!())?;
        // writeln!(f, "number of views: {:>21}", todo!())?;
        // writeln!(f, "schema size: {:>21}", todo!())?;
        // writeln!(f, "data version: {:>21}", todo!())
    }
}