use crate::sqlite::FileFormat;
use crate::sqlite::SchemaFormat;
use crate::sqlite::TextEncoding;
use crate::errors::DatabaseError;


#[allow(unused)]
#[derive(Debug)]
pub struct Header {
    // 0	16	The header string: "SQLite format 3\000"
    pub header: String,
    // 16	2	The database page size in bytes. Must be a power of two between 512 and 32768 inclusive, or the value 1 representing a page size of 65536.
    pub page_size: u32,
    // 18	1	File format write version. 1 for legacy; 2 for WAL.
    pub write_format: FileFormat,
    // 19	1	File format read version. 1 for legacy; 2 for WAL.
    pub read_format: FileFormat,
    // 20	1	Bytes of unused "reserved" space at the end of each page. Usually 0.
    pub reserved_bytes: u8,
    // 21	1	Maximum embedded payload fraction. Must be 64.
    pub max_embedded_payload: u8,
    // 22	1	Minimum embedded payload fraction. Must be 32.
    pub min_embedded_payload: u8,
    // 23	1	Leaf payload fraction. Must be 32.
    pub lead_payload: u8,
    // 24	4	File change counter.
    pub file_change_counter: u32,
    // 28	4	Size of the database file in pages. The "in-header database size".
    pub database_page_count: u32,
    // 32	4	Page number of the first freelist trunk page.
    pub first_page_number_trunk_page: u32,
    // 36	4	Total number of freelist pages.
    pub freelist_page_count: u32,
    // 40	4	The schema cookie.
    pub schema_cookie: u32,
    // 44	4	The schema format number. Supported schema formats are 1, 2, 3, and 4.
    pub schema_format: SchemaFormat,
    // 48	4	Default page cache size.
    pub default_page_cache_size: u32,
    // 52	4	The page number of the largest root b-tree page when in auto-vacuum or incremental-vacuum modes, or zero otherwise.
    pub autovacuum_top_root: u32,
    // 56	4	The database text encoding. A value of 1 means UTF-8. A value of 2 means UTF-16le. A value of 3 means UTF-16be.
    pub text_encoding: TextEncoding,
    // 60	4	The "user version" as read and set by the user_version pragma.
    pub user_version: u32,
    // 64	4	True (non-zero) for incremental-vacuum mode. False (zero) otherwise.
    pub incremental_vacuum_mode: bool,
    // 68	4	The "Application ID" set by PRAGMA application_id.
    pub application_id: u32,
    // 72	20	Reserved for expansion. Must be zero.
    // reserved: Vec<u8>,
    // 92	4	The version-valid-for number.
    pub valid_for_verison: u32,
    // 96	4	SQLITE_VERSION_NUMBER
    pub sqlite_version: u32,
}

impl Header {
    pub fn load(data: &[u8]) -> Result<Self, DatabaseError> {
        Ok(Self {
            header: String::from_utf8(data[0..16].to_vec()).map_err(|e| DatabaseError::StateError(format!("{e:?}")))?,
            page_size: Header::load_page_size(&data[16..=17]),
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
