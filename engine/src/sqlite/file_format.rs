use crate::errors::DatabaseError;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum FileFormat {
    Legacy = 1,
    WAL = 2,
}

impl TryFrom<u8> for FileFormat {
    type Error = DatabaseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(FileFormat::Legacy),
            2 => Ok(FileFormat::WAL),
            _ => Err(DatabaseError::StateError(format!("Unknown file format: {value}")))
        }
    }
}
