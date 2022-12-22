use crate::parser;

pub const T_OUT: &str = r#"$ cd /
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

#[test]
fn parse_terminal_output() {
    // assert that some parser
    // when given
    //  - T_OUT
    // returns a tree structure that displays as
    //  - FS_TREE
    let fs_tree: &str = ""; // replace by result :)
    assert_eq!(fs_tree, FS_TREE_STR);
}

#[test]
fn traverse_filesystem_tree() {
    let mut count = 0;
    let incr_count = || {
        count += 1;
    };
    let commands = parser::terminal(T_OUT);

    assert_eq!(count, 13);
}
