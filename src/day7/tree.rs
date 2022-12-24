use std::{cell::RefCell, rc::Rc};

use crate::{parser::terminal, TERM};

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
    size: u64,
    children: Vec<Node<'a>>,
    parent: Option<&'a Node<'a>>,
}

#[test]
fn try_node() {
    let b = Node {
        size: 2,
        children: vec![],
        parent: None,
    };
    let c = Node {
        size: 3,
        children: vec![],
        parent: None,
    };
    let a = Node {
        size: 0,
        children: vec![b, c],
        parent: None,
    };

    dbg!(a);
    assert!(false);
}

#[test]
fn try_read() {
    let (_, parsed_term) = terminal(TERM).unwrap();
    let mut cmd_iterator = parsed_term.into_iter();
    while let Some(cmd) = cmd_iterator.next() {
        println!("{}", cmd);
    }
    assert!(false);
}
