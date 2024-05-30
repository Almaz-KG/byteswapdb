use crate::errors::DatabaseError;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum PageType {
    // A value of 2 (0x02) means the page is an interior index b-tree page.
    InteriorIndexPage = 2,
    // A value of 5 (0x05) means the page is an interior table b-tree page.
    InteriorTablePage = 5,
    // A value of 10 (0x0a) means the page is a leaf index b-tree page.
    LeafIndexPage = 10,
    // A value of 13 (0x0d) means the page is a leaf table b-tree page.
    LeafTablePage = 13,
}

impl TryFrom<u8> for PageType {
    type Error = DatabaseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Self::InteriorIndexPage),
            5 => Ok(Self::InteriorTablePage),
            10 => Ok(Self::LeafIndexPage),
            13 => Ok(Self::LeafTablePage),
            _ => Err(DatabaseError::StateError(format!(
                "Unknown page type: {value}"
            ))),
        }
    }
}
