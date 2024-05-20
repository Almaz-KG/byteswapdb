
#[derive(Debug)]
pub enum EngineError {
    FileSystemError(String),
    StateError(String),
    InternalError(String),
}
