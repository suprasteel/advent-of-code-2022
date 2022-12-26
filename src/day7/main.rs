mod fs;
mod parser;
mod tree;
use std::fs::read_to_string;

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

fn main() -> std::io::Result<()> {
    let data = read_to_string("./data/day7.dat")?;

    let disk_space = 70_000_000;
    let need_space = 30_000_000;
    let (_, parsed_term) = terminal(data.as_str()).unwrap();
    let mut cmd_iterator = parsed_term.into_iter();
    let tree = build_tree(&mut cmd_iterator, vec![], None);

    let top = if let Ok(tree) = tree {
        let mut top = tree.clone();
        while top.borrow().parent.is_some() {
            top = top.parent().unwrap();
        }
        top
    } else {
        panic!("no top");
    };

    let b = top.borrow();
    let fs_size = b.size();
    let v = top.borrow().to_vec();
    let k100 = v.iter().fold(
        0,
        |acc, (n, s, d)| if *d && s < &100_000 { acc + s } else { acc },
    );

    let free_space = (disk_space - fs_size) as i64;
    let missing_space = need_space - free_space;
    let missing_space = (if missing_space < 0 { 0 } else { missing_space }) as u64;

    let delete_folder = v.into_iter().filter(|elt| elt.1 > missing_space).fold(
        ("".into(), disk_space, false),
        |acc, elt| if elt.1 < acc.1 { elt } else { acc },
    );

    dbg!(fs_size);
    dbg!(k100); // part1 rep
    dbg!(missing_space);
    dbg!(delete_folder); // part2 rep

    Ok(())
}
