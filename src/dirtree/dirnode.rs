use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use super::errors::GenericError;

#[derive(Debug, PartialEq, Clone)]
pub struct DirNodeCore {
    name: String,
    parent: Option<DirNode>,
    children: Vec<DirNode>,
    hidden: bool,
}

pub type DirNodeWrapped = Rc<RefCell<DirNodeCore>>;

#[derive(Debug, PartialEq, Clone)]
pub struct DirNode(DirNodeWrapped);

impl fmt::Display for DirNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.borrow().name)
    }
}

#[allow(unused_must_use, dead_code)]
impl DirNode {
    pub fn new(name: &str) -> DirNode {
        DirNode(Rc::new(RefCell::new(DirNodeCore {
            name: name.to_string(),
            parent: None,
            children: vec![],
            hidden: false,
        })))
    }

    pub fn hide(&mut self) -> Result<(), GenericError> {
        self.0.borrow_mut().hidden = true;
        Ok(())
    }

    pub fn add(&mut self, child: &mut DirNode) -> Result<(), GenericError> {
        match self.has(child) {
            Ok(true) => {
                return Err(GenericError::new(
                    "Could not create directory ${child} - name in use.",
                ))
            }
            _ => (),
        }
        self.0.borrow_mut().children.push(child.to_owned());
        Ok(())
    }

    pub fn prune(&mut self, child: &mut DirNode) -> Result<(), GenericError> {
        let index = self.index_for_child(child.0.borrow().name.as_str());
        match index {
            Some(i) => _ = self.0.borrow_mut().children.swap_remove(i),
            None => (),
        };
        Ok(())
    }

    pub fn move_to(&mut self, parent: &mut DirNode) -> Result<(), GenericError> {
        match parent.has(self) {
            Ok(true) => {
                return Err(GenericError::new(
                    "Can't move ${self},a node with that name alreay exists under ${parent}",
                ))
            }
            _ => (),
        };

        match self.0.borrow().parent {
            // Some(node) => {
            //     node.prune(self);
            // }
            _ => (),
        };

        _ = parent.add(self);
        self.0.borrow_mut().parent = Some(parent.to_owned());
        Ok(())
    }

    pub fn get_child(&mut self, named: &str) -> Option<DirNode> {
        for child in self.0.borrow_mut().children.iter() {
            if child.0.borrow().name == named {
                return Some(child.to_owned());
            }
        }
        None
    }

    pub fn get_parent(&mut self) -> Option<DirNode> {
        self.0.borrow_mut().parent.to_owned()
    }

    fn index_for_child(&mut self, named: &str) -> Option<usize> {
        self.0
            .borrow_mut()
            .children
            .iter()
            .position(|d| d.0.borrow_mut().name == named)
    }

    fn has(&mut self, child: &mut DirNode) -> Result<bool, ()> {
        match self.index_for_child(&child.0.borrow().name) {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}
