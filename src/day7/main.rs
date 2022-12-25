mod fs;
mod parser;
mod tree;
use parser::terminal;
use tree::{build_tree, Parent};

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
    let mut cmd_iterator = parsed_term.into_iter();
    let tree = build_tree(&mut cmd_iterator, vec![], None);

    let size = if let Ok(tree) = tree {
        let mut top = tree.clone();
        while top.parent().is_some() {
            top = top.parent().unwrap();
        }
        let b = top.borrow();
        b.size()
    } else {
        0
    };

    println!("SIZE {}", size);
}
