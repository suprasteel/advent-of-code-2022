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

impl Into<Kind> for File {
    fn into(self) -> Kind {
        Kind::F(self)
    }
}

#[derive(Debug)]
pub struct Directory {
    pub(crate) name: PathBuf,
    children: Vec<Kind>,
}

impl Into<Kind> for Directory {
    fn into(self) -> Kind {
        Kind::D(self)
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
        T: Into<Kind>,
    {
        self.children.push(node.into());
        // push parent self
        self
    }
}

#[derive(Debug)]
pub enum Kind {
    F(File),
    D(Directory),
}

impl Kind {
    pub fn name(&self) -> String {
        let string = |pb: &PathBuf| pb
                .to_str()
                .expect("failed to convert pathbuf to str")
                .to_string();
        match self {
            Kind::F(f) => string(&f.name),
            Kind::D(d) => string(&d.name),
        }
    }
}

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

impl DiskSize for Kind {
    fn size(&self) -> usize {
        match self {
            Kind::F(f) => f.size(),
            Kind::D(d) => d.size(),
        }
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            Kind::F(f) => format!("{} (file, {})", f.name.to_str().unwrap_or(""), f.size()),
            Kind::D(d) => format!("{} (dir)", d.name.to_str().unwrap_or("")),
        };
        write!(f, "- {}", content)
    }
}

#[cfg(test)]
mod test {
    use crate::fs::{DiskSize, Kind};

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

    #[test]
    fn display_node() {
        let file = File::new("filename", 1111);
        let dir = Directory::new("dirname");
        assert_eq!(
            <File as Into<Kind>>::into(file).to_string(),
            "- filename (file, 1111)"
        );
        assert_eq!(
            <Directory as Into<Kind>>::into(dir).to_string(),
            "- dirname (dir)"
        );
    }
}
