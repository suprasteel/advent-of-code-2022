use std::{cell::RefCell, rc::Rc, thread::sleep, time::Duration};

use crate::{
    fs::{DiskSize, Kind},
    parser::{terminal, Cmd},
    TERM,
};

type Rcm<T> = Rc<RefCell<T>>;

#[derive(Debug)]
pub struct Node {
    name: String,
    size: u64,
    children: Vec<Rcm<Self>>,
    parent: Option<Rcm<Self>>,
}

impl Node {
    pub fn new<S: Into<String>>(name: S, size: u64) -> Self {
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

    pub fn size(&self) -> u64 {
        if self.size == 0 {
            self.children
                .iter()
                .fold(0, |acc, c| acc + c.borrow().size())
        } else {
            self.size
        }
    }

    // dirty way
    pub fn to_vec(&self) -> Vec<(String, u64, bool)> {
        let mut list: Vec<(String, u64, bool)> = self
            .children
            .iter()
            .flat_map(|c| c.borrow().to_vec())
            .collect();
        list.push((self.name.clone(), self.size(), self.size == 0));
        list
    }
}

/****
                f1 9100 -> 0, 9100
                f2 1000 -> 0, 1000

            d1          -> 0, 10100 (a = a1, a2, b = b1 + b2)

                f3  800 -> 0, 800
                f3  100 -> 0, 100

            d2          -> 0, 900

        d3            -> 900, 11000 (a = b1

                fa  100 -> 0, 100
                fb  200 -> 0, 200

            da          -> 0, 300

                fc   10 -> 0, 10
                fd    1 -> 0, 1
            db  -> 0, 110
            fe 999 -> 0, 999
        dc  -> 410, 1409
    d4  -> 2310,3309
*****/

pub trait Parent: Clone {
    fn parent(&self) -> Option<Self>;
}

impl Parent for Rc<RefCell<Node>> {
    fn parent(&self) -> Option<Self> {
        self.borrow().parent.clone()
    }
}

fn kind_size(k: &Kind) -> u64 {
    match k {
        Kind::F(f) => f.size as u64,
        Kind::D(_) => 0,
    }
}

pub(crate) fn build_tree<'a, I: Iterator<Item = Cmd>>(
    cmds: &mut I,
    mut path_parts: Vec<String>,
    tree: Option<Rcm<Node>>,
) -> Result<Rcm<Node>, String> {
    let cur_node_name = tree
        .as_ref()
        .map_or_else(|| "[]".into(), |v| v.borrow().name.clone());
    print!(
        "\n\n - build tree. Path is {}, current node is {cur_node_name}\n",
        &path_parts
            .iter()
            .fold("".into(), |a, s| format!("{} {}", a, s))
    );
    // if there is a still a cmd
    if let Some(next_cmd) = cmds.next() {
        match next_cmd {
            Cmd::Cd(path) => {
                print!("\t - command is cd {}", &path);
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
                        path_parts.push("/".into());
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
                    "\t - command is ls {}",
                    &files
                        .iter()
                        .fold("".into(), |acc, f| format!("{}\n\t\t{}", acc, f))
                );
                let tree = tree.expect("try to add children without cwd");
                files.iter().for_each(|f| {
                    let mut new_node = Node::new(f.name(), f.size() as u64);
                    new_node.parent = Some(tree.clone());
                    let new_node = Rc::new(RefCell::new(new_node));
                    tree.borrow_mut().children.push(new_node);
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
fn test_size_fs() {
    let (_, parsed_term) = terminal(TERM).unwrap();
    let mut cmd_iterator = parsed_term.into_iter();
    let tree = build_tree(&mut cmd_iterator, vec![], None);

    let top = if let Ok(tree) = tree {
        let mut top = tree.clone();
        while top.borrow().parent.is_some() {
            top = top.parent().unwrap();
        }
        top
    } else {
        panic!("no top");
    };

    let b = top.borrow();
    let size = b.size();
    let v = top.borrow().to_vec();
    let k100 = v.iter().fold(
        0,
        |acc, (n, s, d)| if *d && s < &100_000 { acc + s } else { acc },
    );

    assert_eq!(size, 48381165_u64);
    assert_eq!(k100, 95437_u64);
}
