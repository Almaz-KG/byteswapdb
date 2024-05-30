/// TODO: Rework the Errors num, make it more real (not too abstract)

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum DatabaseError {
    FileSystemError(String),
    StateError(String),
    InternalError(String),
}
