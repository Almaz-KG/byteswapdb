use super::DatabaseError;

#[derive(Debug)]
pub struct Catalog {
    pub tables_count: u16,
}

impl Catalog {
    pub fn load(_data: &[u8]) -> Result<Self, DatabaseError> {
        todo!()
    }
}
