pub struct File {
    name: String,
    size: u32,
}

impl File {
    pub fn new<S>(name: S, size: u32) -> Self
    where
        S: Into<String>,
    {
        Self {
            name: name.into(),
            size,
        }
    }
}

impl Into<Node> for File {
    fn into(self) -> Node {
        Node::F(self)
    }
}

pub struct Directory {
    name: String,
    children: Vec<Node>,
}

impl Into<Node> for Directory {
    fn into(self) -> Node {
        Node::D(self)
    }
}

impl Directory {
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            name: name.into(),
            children: vec![],
        }
    }

    fn push<T>(&mut self, node: T) -> &mut Self
    where
        T: Into<Node>,
    {
        self.children.push(node.into());
        self
    }
}

pub enum Node {
    F(File),
    D(Directory),
}

trait DiskSize {
    fn size(&self) -> u32;
}

impl DiskSize for File {
    fn size(&self) -> u32 {
        self.size
    }
}

impl DiskSize for Directory {
    fn size(&self) -> u32 {
        self.children
            .iter()
            .fold(0, |sum, child| sum + child.size())
    }
}

impl DiskSize for Node {
    fn size(&self) -> u32 {
        match self {
            Node::F(f) => f.size(),
            Node::D(d) => d.size(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::fs::DiskSize;

    use super::{Directory, File};

    #[test]
    fn build_node() {
        let f1 = File::new("file_1", 1);
        let f2 = File::new("file_2", 10);
        let f3 = File::new("file_3", 100);
        let f4 = File::new("file_4", 1000);

        let mut d1 = Directory::new("dir_1");
        let mut d2 = Directory::new("dir_2");

        d2.push(f3).push(f4);
        d1.push(f1).push(f2).push(d2);

        assert_eq!(d1.size(), 1111);
    }
}
