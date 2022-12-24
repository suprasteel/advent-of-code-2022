use std::{rc::Rc, cell::RefCell};

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

struct Node<'a> {
    size: u32,
    children: Vec<Node<'a>>,
    parent: Option<&'a Node<'a>>,
}

#[test]
fn try_node () {

    let n = Node { size: 1, children: vec![], parent: None };
}




