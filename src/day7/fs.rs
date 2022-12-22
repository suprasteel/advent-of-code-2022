use std::{fmt::Display, path::PathBuf};

#[derive(Debug)]
pub struct File {
    pub name: PathBuf,
    pub size: usize,
}

impl File {
    pub fn new<S>(name: S, size: usize) -> Self
    where
        S: Into<PathBuf>,
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

#[derive(Debug)]
pub struct Directory {
    pub(crate) name: PathBuf,
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
        S: Into<PathBuf>,
    {
        Self {
            name: name.into(),
            children: vec![],
        }
    }

    pub fn push<T>(&mut self, node: T) -> &mut Self
    where
        T: Into<Node>,
    {
        self.children.push(node.into());
        self
    }
}

#[derive(Debug)]
pub enum Node {
    F(File),
    D(Directory),
}

/*
impl Node {
    fn name(&self) -> &str {
        match self {
            Self::F(f) => f.name.into(),
            Self::D(d) => d.path.as_ref(),
        }
    }
}
*/

pub trait DiskSize {
    fn size(&self) -> usize;
}

impl DiskSize for File {
    fn size(&self) -> usize {
        self.size
    }
}

impl DiskSize for Directory {
    fn size(&self) -> usize {
        self.children
            .iter()
            .fold(0, |sum, child| sum + child.size())
    }
}

impl DiskSize for Node {
    fn size(&self) -> usize {
        match self {
            Node::F(f) => f.size(),
            Node::D(d) => d.size(),
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            Node::F(f) => format!("{} (file, {})", f.name.to_str().unwrap_or(""), f.size()),
            Node::D(d) => format!("{} (dir)", d.name.to_str().unwrap_or("")),
        };
        write!(f, "- {}", content)
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
