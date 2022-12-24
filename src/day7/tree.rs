use std::{cell::RefCell, rc::Rc};

use crate::{
    parser::{terminal, Cmd},
    TERM,
};

/*type NodeRef<T> = Rc<RefCell<Node<T>>>;

struct Node<T> {
    content: T,
    children: Vec<NodeRef<T>>,
    parent: Option<NodeRef<T>>
}*/

/*impl<T> Node<T> {
    pub fn new(t: T) -> Self {
        Self {
            content: t,
            children: vec![],
            parent: None,
        }
    }

    pub fn insert(&mut self, t: T) {
        let mut new_node = Node::new(t);
        new_node.set_parent(Rc::new(RefCell::new(self)));
        self.children.push();
    }

    fn set_parent(&mut self, parent: NodeRef<T>) -> Option<NodeRef<T>> {
        self.parent.replace(parent)
    }
}*/

/*


let dir1 = Dir();
let dir2 = Dir();
dir1.insert(file);
dir1.insert(dir2);


*/
#[derive(Debug)]
struct Node<'a> {
    name: String,
    size: u64,
    children: Vec<Node<'a>>,
    parent: Option<&'a Node<'a>>,
}

impl<'a> Node<'a> {

    fn new<S: Into<String>>(name: S, size: u64) -> Self {
        Self {
            name: name.into(), size, children: vec![], parent: None,
        }
    }

    fn has_name(&self, n: &str) -> bool {
        self.name == n
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



#[test]
fn try_read() {
    let tree: Option<Node> = None;
    let (_, parsed_term) = terminal(TERM).unwrap();
    let mut cmd_iterator = parsed_term.into_iter();
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