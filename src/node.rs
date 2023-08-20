use super::Error;
use std::{
    collections::{btree_map::Entry, BTreeMap},
    ops::DerefMut,
};

#[derive(Default)]
pub(crate) struct Node {
    children: BTreeMap<String, Box<Node>>,
}

impl Node {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    fn lookup(&mut self, path: &[&str]) -> Result<&mut Self, String> {
        let mut node = self;
        for (i, name) in path.iter().enumerate() {
            if let Some(child_node) = node.children.get_mut(*name) {
                node = child_node.deref_mut();
            } else {
                return Err(path[0..i].join("/"));
            }
        }

        Ok(node)
    }

    fn list(&self, indent: usize) {
        for (child, node) in self.children.iter() {
            println!("{:>indent$}{child}", "");
            node.list(indent + 2);
        }
    }

    pub(crate) fn execute(&mut self, cmd: &str) -> Result<(), Error> {
        fn check_param_count(parts: &[&str], expected: usize) -> Result<(), Error> {
            if parts.len() != 1 + expected {
                Err(Error::ParamCount(expected, parts.join(" ")))
            } else {
                Ok(())
            }
        }

        let parts: Vec<&str> = cmd.split_whitespace().collect();

        match parts[0] {
            "CREATE" => {
                check_param_count(&parts, 1)?;
                let mut path: Vec<&str> = parts[1].split('/').collect();
                // Unwrap ok here, check_param_count ensures a path was given
                let child: String = path.pop().unwrap().into();

                let parent = self.lookup(&path).map_err(|missing| Error::Missing {
                    op: "create",
                    path: parts[1].into(),
                    missing,
                })?;
                match parent.children.entry(child) {
                    Entry::Vacant(e) => e.insert(Default::default()),
                    Entry::Occupied(_) => {
                        return Err(Error::Exists {
                            op: "create",
                            path: parts[1].into(),
                        });
                    }
                };
            }
            "DELETE" => {
                check_param_count(&parts, 1)?;
                let mut path: Vec<&str> = parts[1].split('/').collect();
                // Unwrap ok here, check_param_count ensures a path was given
                let child: &str = path.pop().unwrap();

                let parent = self.lookup(&path).map_err(|missing| Error::Missing {
                    op: "delete",
                    path: parts[1].into(),
                    missing,
                })?;
                if parent.children.contains_key(child) {
                    parent.children.remove(child);
                } else {
                    return Err(Error::Missing {
                        op: "delete",
                        path: parts[1].into(),
                        missing: parts[1].into(),
                    });
                }
            }
            "LIST" => {
                check_param_count(&parts, 0)?;
                self.list(0);
            }
            "MOVE" => {
                check_param_count(&parts, 2)?;
                let mut from_path: Vec<&str> = parts[1].split('/').collect();
                // Unwrap ok here, check_param_count ensures a path was given
                let child: String = from_path.pop().unwrap().into();

                let from_parent = self.lookup(&from_path).map_err(|missing| Error::Missing {
                    op: "move",
                    path: parts[1].into(),
                    missing,
                })?;
                if let Some(node) = from_parent.children.remove(&child) {
                    let to_path: Vec<&str> = parts[2].split('/').collect();
                    let to_parent = self.lookup(&to_path).map_err(|missing| Error::Missing {
                        op: "move",
                        path: parts[2].into(),
                        missing,
                    })?;

                    match to_parent.children.entry(child) {
                        Entry::Vacant(e) => e.insert(node),
                        Entry::Occupied(_) => {
                            return Err(Error::Exists {
                                op: "move",
                                path: parts[1].into(),
                            });
                        }
                    };
                } else {
                    return Err(Error::Missing {
                        op: "move",
                        path: parts[1].into(),
                        missing: parts[1].into(),
                    });
                }
            }
            _ => return Err(Error::UnknownCommand(cmd.into())),
        }
        Ok(())
    }
}
