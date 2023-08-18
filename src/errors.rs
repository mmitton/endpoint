use std::fmt;

#[derive(Debug, Clone)]
pub enum ApplicationError {
    UsageError,
    MissingFileError,
}

#[derive(Debug, Clone)]
pub struct UsageError;
impl fmt::Display for UsageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Usage: endpoint <filename>")
    }
}
impl From<UsageError> for ApplicationError {
    fn from(_: UsageError) -> Self {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct MissingFileError;
impl fmt::Display for MissingFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "That file could not be found.")
    }
}
impl From<MissingFileError> for ApplicationError {
    fn from(_: MissingFileError) -> Self {
        todo!()
    }
}
