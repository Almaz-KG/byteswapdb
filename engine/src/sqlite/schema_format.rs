use crate::errors::DatabaseError;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum SchemaFormat {
    Format1 = 1,
    Format2 = 2,
    Format3 = 3,
    Format4 = 4,
}

impl TryFrom<u32> for SchemaFormat {
    type Error = DatabaseError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(SchemaFormat::Format1),
            2 => Ok(SchemaFormat::Format2),
            3 => Ok(SchemaFormat::Format3),
            4 => Ok(SchemaFormat::Format4),
            _ => Err(DatabaseError::StateError(format!("Unsupported schema format: {value}")))
        }
    }
}
