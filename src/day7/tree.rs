use std::{cell::RefCell, rc::Rc};

use crate::{
    fs::{DiskSize, Kind},
    parser::{terminal, Cmd},
    TERM,
};

type Rcm<T> = Rc<RefCell<T>>;

#[derive(Debug)]
struct Node {
    name: String,
    size: u64,
    children: Vec<Rcm<Self>>,
    parent: Option<Rcm<Self>>,
}

impl Node {
    fn new<S: Into<String>>(name: S, size: u64) -> Self {
        Self {
            name: name.into(),
            size,
            children: vec![],
            parent: None,
        }
    }

    fn has_name<S: AsRef<str>>(&self, name: S) -> bool {
        self.name == name.as_ref()
    }
}

trait Parent: Clone {
    fn parent(self) -> Option<Self>;
}

impl Parent for Rc<RefCell<Node>> {
    fn parent(self) -> Option<Self> {
        self.borrow().parent.clone()
    }
}

#[test]
fn try_node() {
    let _b = Node::new("b", 2);
    let _c = Node::new("c", 3);
    let a = Node::new("a", 0);

    dbg!(a);
    assert!(false);
}

fn kind_size(k: &Kind) -> u64 {
    match k {
        Kind::F(f) => f.size as u64,
        Kind::D(_) => 0,
    }
}

fn build_tree<'a, I: Iterator<Item = Cmd>>(
    cmds: &mut I,
    mut path_parts: Vec<String>,
    tree: Option<Rcm<Node>>,
) -> Result<Rcm<Node>, String> {
    print!("\n build tree: ");
    // if there is a still a cmd
    if let Some(next_cmd) = cmds.next() {
        match next_cmd {
            Cmd::Cd(path) => {
                print!("cd {}", &path);
                // it is a cd
                let node = match path.as_str() {
                    "/" => {
                        // to root: if none, create it, otherwise go to the top
                        let mut root = tree
                            .clone()
                            .unwrap_or_else(|| Rc::new(RefCell::new(Node::new("/", 0))));
                        while root.borrow().parent.is_some() {
                            root = root.parent().unwrap();
                            path_parts.pop();
                        }
                        // return build_tree(cmds, path_parts, Some(root));
                        root
                    }
                    ".." => {
                        let parent = match tree {
                            Some(tree) => {
                                let parent = tree.borrow().parent.clone();
                                match parent {
                                    Some(p) => Ok(p.clone()),
                                    None => Err(format!(
                                        "No parent found on node {}",
                                        tree.borrow().name
                                    )),
                                }
                            }
                            None => Err("Trying to move to parent directy with no cwd".to_string()),
                        }?;
                        path_parts.pop();
                        // return build_tree(cmds, path_parts, Some(parent));
                        parent
                    }
                    _ => {
                        let node = match tree {
                            Some(tree) => Ok(tree),
                            None => Err("trying to cd while cwd is not set"),
                        }?;
                        let node = node.borrow();
                        let child = node.children.iter().find(|c| c.borrow().has_name(&path));
                        // return build_tree(cmds, path_parts, Some(parent));
                        let c = match child {
                            Some(c) => Ok(c.clone()),
                            None => Err(format!(
                                "No child matching {} found on node {}",
                                &path, node.name
                            )),
                        }?;
                        path_parts.push(path);
                        c
                    }
                };
                build_tree(cmds, path_parts, Some(node))
            }
            Cmd::Ls(files) => {
                print!(
                    "ls {}",
                    &files
                        .iter()
                        .fold("".into(), |acc, f| format!("{}, {}", acc, f))
                );
                let tree = tree.expect("try to add children without cwd");
                files.iter().for_each(|f| {
                    
                    tree.borrow_mut()
                        .children
                        .push(Rc::new(RefCell::new(Node::new(f.name(), f.size() as u64))));
                });
                build_tree(cmds, path_parts, Some(tree))
            }
        }
    } else {
        tree.ok_or_else(|| "Empty tree".into())
    }
}

// build_tree(cmds) -> tree
// build_node(cmds, tree, node) -> node
//

#[test]
fn try_read() {
    let (_, parsed_term) = terminal(TERM).unwrap();
    dbg!(&parsed_term);
    let mut cmd_iterator = parsed_term.into_iter();

    let tree = build_tree(&mut cmd_iterator, vec![], None);

    match tree {
        Ok(r) => {
            dbg!(r);
        }
        e => println!("Oups: {:?}", e),
    }

    assert!(false);
}
