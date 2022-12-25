use std::rc::Rc;

use crate::{
    fs::{Kind, DiskSize},
    parser::{Cmd, terminal}, TERM,
};

#[derive(Debug)]
struct Node {
    name: String,
    size: u64,
    children: Vec<Rc<Node>>,
    parent: Option<Rc<Node>>,
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

#[test]
fn try_node() {
    let b = Node::new("b", 2);
    let c = Node::new("c", 3);
    let a = Node::new("a", 0);

    dbg!(a);
    assert!(false);
}

/*
// cd
// ls
// cd
// cd
// cd
// ls (files)

(cmd) -> (cur_cd)

*/

fn kind_size(k: &Kind) -> u64 {
    match k {
        Kind::F(f) => f.size as u64,
        Kind::D(_) => 0,
    }
}

fn build_tree<'a, I: Iterator<Item = Cmd>>(
    cmds: &mut I,
    mut path_parts: Vec<String>,
    tree: Option<Rc<Node>>,
) -> Result<Rc<Node>, String> {
    // if there is a still a cmd
    if let Some(next_cmd) = cmds.next() {
        match next_cmd {
            Cmd::Cd(path) => {
                // it is a cd
                let node = match path.as_str() {
                    "/" => {
                        // to root
                        let mut root = tree.clone().unwrap_or_else(|| Rc::new(Node::new("/", 0)));
                        while root.parent.is_some() {
                            root = root.parent.clone().unwrap();
                            path_parts.pop();
                        }
                        // return build_tree(cmds, path_parts, Some(root));
                        root
                    }
                    ".." => {
                        let parent = tree
                            .expect("trying to go up while cwd is not set")
                            .parent
                            .expect("trying to go up on the root");
                        path_parts.pop();
                        // return build_tree(cmds, path_parts, Some(parent));
                        parent
                    }
                    _ => {
                        path_parts.push(path);
                        let tree = tree.expect("trying to cd while cwd is not set");
                        // should log dir name and path
                        let child = tree
                            .children
                            .iter()
                            .find(|c| c.has_name(path))
                            .expect("directory not found in tree");
                        // return build_tree(cmds, path_parts, Some(parent));
                        *child
                    }
                };
                build_tree(cmds, path_parts, Some(node))
            }
            Cmd::Ls(files) => {
                let tree = tree.expect("try to add children without cwd");
                files.iter().for_each(|f| tree.children.push(Rc::new(Node::new(f.name(), f.size() as u64))));
                build_tree(cmds, path_parts, Some(tree))
            },
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
    let tree: Option<Node> = None;
    let (_, parsed_term) = terminal(TERM).unwrap();
    let mut cmd_iterator = parsed_term.into_iter();
    // build_tree(cmd_iterator, vec![]);
    while let Some(cmd) = cmd_iterator.next() {
        match cmd {
            Cmd::Cd(p) => {
                println!("cd to {}", p);
                let dir = Node::new(p, 0);
            }
            Cmd::Ls(list) => {
                list.iter().for_each(|v| println!("Ls result is: \n{}", v));
            }
        }
    }
    assert!(false);
}
