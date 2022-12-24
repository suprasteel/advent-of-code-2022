mod fs;
mod parser;
mod tree;
use std::rc::Rc;

use fs::Directory;
use parser::{terminal, Cmd};

pub const TERM: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"#;

const FS_TREE_STR: &str = r#"- / (dir)
  - a (dir)
    - e (dir)
      - i (file, size=584)
    - f (file, size=29116)
    - g (file, size=2557)
    - h.lst (file, size=62596)
  - b.txt (file, size=14848514)
  - c.dat (file, size=8504156)
  - d (dir)
    - j (file, size=4060174)
    - d.log (file, size=8033020)
    - d.ext (file, size=5626152)
    - k (file, size=7214296)"#;



fn main() {
    let (_, parsed_term) = terminal(TERM).unwrap();
    parsed_term.iter().for_each(|v| println!("{}", v));

    let mut currentd: Option<Rc<Directory>> = None;

/*
let cmd = Cmd::Ls(vec![]);
    match cmd {
        Cmd::Cd(root) if root == "/" => {
            let cur_dir = match currentd {
                Some(c) => {
                    loop {
                        match c.parent() {
                            None => break,
                            Some(parent) => 
                        }
                        
                    }
                },
                None => Directory::new(root)
            }
    },
        Cmd::Cd(to_dir) => {
            let cur_dir = match currentd {
                Some(c) => {c.push(Directory::new(to_dir)); c},
                None => Directory::new(to_dir)
            }
        },
        Cmd::Ls(a) => {}
    }
    */
}

