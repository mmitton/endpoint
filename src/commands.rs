#![allow(dead_code, unused_imports)]

use crate::dirtree::DirNode;
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
    type Err = ();
    fn from_str(input: &str) -> Result<AllowedCommands, Self::Err> {
        match input {
            "CREATE" => Ok(AllowedCommands::CREATE),
            "DELETE" => Ok(AllowedCommands::DELETE),
            "MOVE" => Ok(AllowedCommands::MOVE),
            "LIST" => Ok(AllowedCommands::LIST),
            "NOOP" => Ok(AllowedCommands::NOOP),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct Command {
    operation: AllowedCommands,
    operand_1: Option<DirNode>,
    operand_2: Option<DirNode>,
}
impl Command {
    pub fn new() -> Command {
        return Command {
            operation: AllowedCommands::NOOP,
            operand_1: None,
            operand_2: None,
        };
    }
}
