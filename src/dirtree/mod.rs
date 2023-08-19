#[allow(unused_must_use, dead_code)]
use std::{borrow::BorrowMut, str::FromStr};

mod commands;
mod dirnode;
mod errors;
use commands::AllowedCommands;
use dirnode::DirNode;
pub use errors::GenericError;

#[derive(Debug)]
pub struct DirTree {
    root: DirNode,
}
impl DirTree {
    pub fn new() -> DirTree {
        let mut root_node = DirNode::new("root");
        _ = root_node.hide();
        DirTree { root: root_node }
    }

    fn tokenize_path(s: &str) -> Vec<&str> {
        s.split("/").map(|x| x).collect()
    }

    fn tokenize_instruction(s: &str) -> Vec<&str> {
        s.split_whitespace().map(|x| x).collect()
    }

    pub fn execute(&mut self, instruction: String) -> Result<(), GenericError> {
        let instruction_tokens = Self::tokenize_instruction(&instruction);
        let command = match AllowedCommands::from_str(instruction_tokens[0]) {
            Ok(c) => c,
            Err(e) => return Err(e),
        };
        match command {
            AllowedCommands::CREATE => {
                if instruction_tokens.len() < 2 {
                    return Err(GenericError::new("CREATE command requires an argument."));
                }
                let mut current_parent = &self.root;
                let mut current_node: DirNode;
                let mut node: DirNode;
                let path_tokens = Self::tokenize_path(instruction_tokens[1]);
                for path_token in path_tokens {
                    node = DirNode::new(path_token);
                    match node.move_to(current_parent.to_owned().borrow_mut()) {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    }
                    current_node = node;
                    current_parent = &current_node;
                }
                Ok(())
            }

            // AllowedCommands::DELETE => {
            //     if instruction_tokens.len() < 2 {
            //         return Err(GenericError::new("DELETE command requires an argument."));
            //     }
            //     let mut node: &DirNode = &self.root;
            //     let path_tokens = Self::tokenize_path(instruction_tokens[1]);
            //     for path_token in path_tokens {
            //         let child = node.get_child(path_token);
            //         match child {
            //             None => (),
            //             Some(d) => node = d,
            //         };
            //     }
            //     let mut parent = match node.get_parent() {
            //         None => return Ok(()),
            //         Some(n) => n,
            //     };

            //     parent.prune((*node).borrow_mut())
            // }

            // AllowedCommands::MOVE => {
            //     if instruction_tokens.len() < 2 {
            //         return Err(GenericError::new("MOVE command requires 2 arguments."));
            //     }
            //     let src_tokens = Self::tokenize_path(instruction_tokens[1]);
            //     let dest_tokens = Self::tokenize_path(instruction_tokens[2]);
            //     Ok(())
            // }
            // AllowedCommands::LIST => Ok(()),
            _ => return Err(GenericError::new("Unrecognized command!")),
        }
    }
}
