use super::errors::GenericError;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum AllowedCommands {
    CREATE,
    DELETE,
    MOVE,
    LIST,
    NOOP,
}

impl FromStr for AllowedCommands {
    type Err = GenericError;
    fn from_str(input: &str) -> Result<AllowedCommands, GenericError> {
        match input {
            "CREATE" => Ok(AllowedCommands::CREATE),
            "DELETE" => Ok(AllowedCommands::DELETE),
            "MOVE" => Ok(AllowedCommands::MOVE),
            "LIST" => Ok(AllowedCommands::LIST),
            "NOOP" => Ok(AllowedCommands::NOOP),
            _ => Err(GenericError::new("${input} is not a recognized command.")),
        }
    }
}
