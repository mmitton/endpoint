mod commands;
mod dirnode;
mod errors;
use std::str::FromStr;

use dirnode::DirNode;
pub use errors::GenericError;

use self::{commands::AllowedCommands, dirnode::DirNodeFunctions};

fn tokenize_path(s: &str) -> Vec<&str> {
    s.split("/").map(|x| x).collect()
}

fn tokenize_instruction(s: &str) -> Vec<&str> {
    s.split_whitespace().map(|x| x).collect()
}

#[derive(Debug)]
pub struct DirTree {
    root: DirNode,
}
impl DirTree {
    pub fn new() -> DirTree {
        let root_node = DirNode::spawn("root");
        DirTree { root: root_node }
    }

    pub fn execute(&mut self, instruction: String) -> Result<(), GenericError> {
        let instruction_tokens = tokenize_instruction(&instruction);
        let command = match AllowedCommands::from_str(instruction_tokens[0]) {
            Ok(allowed) => allowed,
            Err(e) => return Err(e),
        };
        match command {
            AllowedCommands::CREATE => {
                let path_tokens = tokenize_path(instruction_tokens[1]);
                let mut current_parent = self.root.clone();
                for path_token in path_tokens {
                    match current_parent.has_child(path_token) {
                        Some(node) => {
                            current_parent = node.clone();
                        }
                        None => {
                            let child = DirNode::spawn(path_token);
                            _ = current_parent.adopt(child.clone());
                            current_parent = child.clone();
                        }
                    }
                }
                Ok(())
            }

            AllowedCommands::LIST => {
                fn depth_first(mut node: DirNode, depth: usize) {
                    if depth > 0 {
                        println!("{}{}", String::from(" ").repeat(depth - 1), node.name());
                    }
                    let child_count = node.children().len();
                    let mut passes = 0;
                    while child_count > passes {
                        let mut children: Vec<DirNode> = node.children().clone();
                        children.sort();
                        depth_first(children[passes].clone(), depth + 1);
                        passes += 1;
                    }
                }
                depth_first(self.root.clone(), 0);
                Ok(())
            }

            AllowedCommands::MOVE => {
                let src_tokens = tokenize_path(instruction_tokens[1]);
                let dst_tokens = tokenize_path(instruction_tokens[2]);
                let mut node_to_move = self.root.clone();
                let mut new_parent = self.root.clone();
                for token in src_tokens.clone() {
                    let result = node_to_move.has_child(token);
                    match result {
                        Some(node) => {
                            node_to_move = node.clone();
                        }
                        _ => (),
                    }
                }
                if node_to_move.name() != src_tokens.clone().last().unwrap().to_string() {
                    return Ok(());
                }
                for token in dst_tokens.clone() {
                    let result = new_parent.has_child(token);
                    match result {
                        Some(node) => {
                            new_parent = node.clone();
                        }
                        _ => (),
                    }
                }
                if new_parent.name() != dst_tokens.clone().last().unwrap().to_string() {
                    return Ok(());
                }
                _ = new_parent.adopt(node_to_move);
                Ok(())
            }

            AllowedCommands::DELETE => {
                let path_tokens = tokenize_path(instruction_tokens[1].clone());
                let mut current_node = self.root.clone();
                let mut last_checked_name: &str = "root";
                for token in path_tokens.clone() {
                    last_checked_name = token;
                    let result = current_node.has_child(token);
                    match result {
                        Some(node) => {
                            current_node = node.clone();
                        }
                        _ => (),
                    }
                }
                if current_node.name() != path_tokens.clone().last().unwrap().to_string() {
                    let path = instruction_tokens[1].clone();
                    return Err(GenericError::new(
                        format!("Cannot delete {path} - {last_checked_name} does not exist")
                            .as_str(),
                    ));
                }
                match current_node.parent() {
                    Some(node) => {
                        _ = node.clone().emancipate(current_node);
                    }
                    _ => (),
                }
                return Ok(());
            }

            _ => return Err(GenericError::new("Unrecognized command!")),
        }
    }
}
