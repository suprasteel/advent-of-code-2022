use std::rc::Rc;

struct Node<T> {
    content: T,
    children: Vec<Rc<Node<T>>>,
    parent: Option<Rc<Node<T>>>
}

impl<T> Node<T> {
    pub fn new(t: T) -> Self {
        Self {
            content: t,
            children: vec![],
            parent: None,
        }
    }

    pub fn insert(self, t: Rc<Node<T>>) {
        t.as_ref().set_parent(Rc::new(self));
        self.children.push(t);
    }

    pub fn set_parent(&self, t: Rc<Node<T>>) -> Option<Rc<Node<T>>> {
        self.parent.replace(t)
    }
}

#[test]
fn tree() {}
