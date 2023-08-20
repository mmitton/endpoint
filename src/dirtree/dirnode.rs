use std::cell::RefCell;
use std::rc::Rc;

use super::GenericError;

pub type DirNode = Rc<RefCell<DirNodeCore>>;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct DirNodeCore {
    name: String,
    me: Option<DirNode>,
    parent: Option<DirNode>,
    children: Vec<DirNode>,
}

pub trait DirNodeFunctions {
    fn spawn(name: &str) -> DirNode;
    fn name(&mut self) -> String;
    fn parent(&mut self) -> Option<DirNode>;
    fn children(&mut self) -> Vec<DirNode>;
    fn has_child(&mut self, named: &str) -> Option<DirNode>;
    fn adopt(&mut self, child: DirNode) -> Result<(), GenericError>;
    fn emancipate(&mut self, child: DirNode) -> Result<(), GenericError>;
}
impl DirNodeFunctions for DirNode {
    fn spawn(name: &str) -> DirNode {
        let node = Rc::new(RefCell::new(DirNodeCore {
            name: name.to_string(),
            me: None,
            parent: None,
            children: vec![],
        }));
        node.borrow_mut().me = Some(node.clone());
        return node;
    }

    fn name(&mut self) -> String {
        self.borrow().name.clone()
    }

    fn parent(&mut self) -> Option<DirNode> {
        self.clone().borrow().parent.clone()
    }

    fn children(&mut self) -> Vec<DirNode> {
        self.clone().borrow().children.clone()
    }

    fn has_child(&mut self, name: &str) -> Option<DirNode> {
        for node in self.children() {
            if node.borrow().name == String::from(name) {
                return Some(node.clone());
            }
        }
        None
    }

    fn adopt(&mut self, child: DirNode) -> Result<(), GenericError> {
        _ = self.borrow_mut().add_child(child.clone());
        let old_parent = child.borrow_mut().parent.clone();
        if old_parent.is_some() {
            _ = old_parent.unwrap().borrow_mut().remove_child(child.clone())
        }
        _ = child.borrow_mut().set_parent(self.clone());
        Ok(())
    }

    fn emancipate(&mut self, child: DirNode) -> Result<(), GenericError> {
        _ = child.borrow_mut().unset_parent();
        _ = self.borrow_mut().remove_child(child.clone());
        Ok(())
    }
}

impl DirNodeCore {
    fn add_child(&mut self, node: DirNode) -> Result<(), GenericError> {
        self.children.push(node.clone());
        Ok(())
    }

    fn remove_child(&mut self, node: DirNode) -> Result<(), GenericError> {
        let result = self.index_for_child(node);
        if result.is_some() {
            self.children.remove(result.unwrap());
        }
        Ok(())
    }

    fn set_parent(&mut self, node: DirNode) -> Result<(), GenericError> {
        self.parent = Some(node.clone());
        Ok(())
    }

    fn unset_parent(&mut self) -> Result<(), GenericError> {
        self.parent = None;
        Ok(())
    }

    fn index_for_child(&mut self, node: DirNode) -> Option<usize> {
        self.children
            .iter()
            .position(|child| child.borrow().name == node.borrow().name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dirnode_new() {
        let node = Rc::new(RefCell::new(DirNodeCore {
            name: String::from("gary"),
            me: None,
            parent: None,
            children: vec![],
        }));
        node.borrow_mut().me = Some(node.clone());
        let another_node = DirNode::spawn("jon");
        assert_ne!(node.borrow().name, another_node.borrow().name);
    }

    #[test]
    fn dirnode_add_child() {
        let parent_node = DirNode::spawn("parent");
        let child_node = DirNode::spawn("child");
        let _ = parent_node.borrow_mut().add_child(child_node.clone());
        assert_eq!(
            child_node.borrow().name,
            parent_node.borrow().children[0].borrow().name
        );
    }

    #[test]
    fn dirnode_index_for_child() {
        let parent_node = DirNode::spawn("parent");
        let child_node = DirNode::spawn("child");
        let _ = parent_node.borrow_mut().add_child(child_node.clone());
        assert_eq!(
            0,
            parent_node
                .borrow_mut()
                .index_for_child(child_node)
                .unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn dirnode_no_parent() {
        let node = DirNode::spawn("node");
        _ = node.borrow().parent.clone().unwrap();
    }
}
