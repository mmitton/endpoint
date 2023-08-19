use std::fmt;

#[derive(Debug, Clone)]
pub struct GenericError {
    error: String,
}
impl GenericError {
    pub fn new(error_string: &str) -> GenericError {
        GenericError {
            error: error_string.to_string(),
        }
    }
}
impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}
