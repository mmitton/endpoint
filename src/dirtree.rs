#![allow(dead_code)]

use std::cell::RefCell;
use std::error::Error;
use std::fmt;
use std::io::Split;
use std::rc::Rc;
use std::str::FromStr;
use std::str::SplitWhitespace;
use std::vec;

use crate::commands::AllowedCommands;

#[derive(Debug)]
pub struct DirTree {
    root: Rc<RefCell<DirNode>>,
}
impl DirTree {
    pub fn new() -> DirTree {
        let root: Rc<RefCell<DirNode>> = Rc::new(RefCell::new(DirNode::new("root")));
        DirTree { root: root }
    }

    fn tokenize_path(path: &str) -> Vec<String> {
        path.split("/").map(|x| x.to_string()).collect()
    }

    pub fn execute(&mut self, instruction: String) -> Result<(), DirTreeError> {
        let mut tokens = instruction.split_whitespace();
        let command = match tokens.next() {
            Some(text) => AllowedCommands::from_str(text),
            None => return Err(DirTreeError::new("No command found.")),
        };

        match command.unwrap() {
            AllowedCommands::CREATE => match tokens.next() {
                Some(path) => {
                    let tokens = DirTree::tokenize_path(path);
                    let mut i = tokens.iter();
                    let root_path_token = match i.next() {
                        Some(text) => text.to_string(),
                        None => {
                            return Err(DirTreeError::new(
                                "Impossible missing root_path_token error.",
                            ))
                        }
                    };
                    match self.root.borrow().child_named(&root_path_token.clone()) {
                        Some(_) => {
                            return Err(DirTreeError::new(
                                "Cannot create ${root_path_token}, directory already exists.",
                            ))
                        }
                        None => {}
                    }
                }
                None => Err(DirTreeError::new("CREATE command requires an argument.")),
            },

            AllowedCommands::DELETE => match tokens.next() {
                Some(_path) => {
                    // let path_tokens = path.split("/");
                    Ok(())
                }
                None => Err(DirTreeError::new("DELETE command requires an argument.")),
            },

            AllowedCommands::MOVE => match tokens.next() {
                Some(_path) => {
                    // let path_tokens = path.split("/");
                    Ok(())
                }
                None => Err(DirTreeError::new("MOVE command requires two arguments.")),
            },

            AllowedCommands::LIST => Ok(()),

            _ => return Err(DirTreeError::new("Unrecognized command!")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DirTreeError {
    error: String,
}
impl DirTreeError {
    fn new(error_string: &str) -> DirTreeError {
        DirTreeError {
            error: error_string.to_string(),
        }
    }
}
impl fmt::Display for DirTreeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DirNode {
    name: String,
    parent: Option<Rc<RefCell<DirNode>>>,
    children: Vec<Rc<RefCell<DirNode>>>,
    hidden: bool,
}
impl DirNode {
    pub fn new(name: &str) -> DirNode {
        return DirNode {
            name: name.to_string(),
            parent: None,
            children: vec![],
            hidden: false,
        };
    }

    pub fn set_name(&mut self, new_name: String) -> Result<(), ()> {
        match self.child_named(&new_name) {
            Some(_) => Err(()),
            None => {
                self.name = new_name;
                Ok(())
            }
        }
    }

    pub fn add_child(&mut self, new_node: Rc<RefCell<DirNode>>) {
        self.children.push(new_node);
    }

    pub fn remove_child(&mut self, dir_node: &mut DirNode) {
        let mut index = 0;
        for child_node in self.children.iter() {
            if dir_node.name == child_node.borrow().name {
                break;
            }
            index += 1;
        }
        self.children.swap_remove(index);
    }

    pub fn child_named(&mut self, name: &String) -> Option<Rc<RefCell<DirNode>>> {
        for child_node in self.children.iter() {
            if child_node.borrow().name == name.to_owned() {
                return Some(child_node.clone());
            }
        }
        None
    }

    pub fn set_parent(&mut self, new_parent_node: Rc<RefCell<DirNode>>) {
        match self.parent.clone() {
            Some(inner_rc) => inner_rc.borrow_mut().remove_child(self),
            None => (),
        }
        self.parent = Some(new_parent_node.clone());
    }
}
