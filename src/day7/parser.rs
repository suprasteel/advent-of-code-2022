use std::{fmt::Display, path::PathBuf};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, digit1, line_ending},
    combinator::{map, map_res},
    multi::many0,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

use crate::fs::{Directory, File, Kind};

pub(crate) fn path(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| match c as u8 {
        b'a'..=b'z' => true,
        b'A'..=b'Z' => true,
        b'_' | b'-' | b'/' | b'.' => true,
        _ => false,
    })(input)
}

pub(crate) fn pathbuf(input: &str) -> IResult<&str, PathBuf> {
    map(path, Into::into)(input)
}

pub(crate) fn usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |b: &str| b.parse::<usize>())(input)
}

pub(crate) fn file(input: &str) -> IResult<&str, File> {
    map(separated_pair(usize, char(' '), path), |(size, name)| {
        File::new(name, size)
    })(input)
}

pub(crate) fn file_node(input: &str) -> IResult<&str, Kind> {
    map(file, |file| file.into())(input)
}

pub(crate) fn dir(input: &str) -> IResult<&str, Directory> {
    let parser = separated_pair(tag("dir"), char(' '), path);
    map(parser, |(_, path)| Directory::new(path))(input)
}
pub(crate) fn dir_node(input: &str) -> IResult<&str, Kind> {
    map(dir, |d| d.into())(input)
}
pub(crate) fn nodes(input: &str) -> IResult<&str, Vec<Kind>> {
    many0(preceded(line_ending, alt((file_node, dir_node))))(input)
}
#[derive(Debug)]
pub(crate) enum Cmd {
    Ls(Vec<Kind>),
    Cd(String),
}

impl Display for Cmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cmd::Ls(ret) => format!("ls returns {} files and dirs", ret.len()),
                Cmd::Cd(arg) => format!("cd to directory {}", arg),
            }
        )
    }
}

pub(crate) fn ls(input: &str) -> IResult<&str, Vec<Kind>> {
    preceded(tag("ls"), nodes)(input)
}

pub(crate) fn cd(input: &str) -> IResult<&str, String> {
    preceded(tag("cd "), map(path, |str| str.to_string()))(input)
}

/// Split a single CD command in many commands to be able to build corresponding nodes
pub(crate) fn split_cd(cd: Cmd) -> (Cmd, Option<Cmd>) {
    assert!(matches!(cd, Cmd::Cd(_)));
    let splitted = match cd {
        Cmd::Cd(ref s) => s.split_once("/"),
        _ => None,
    };

    match splitted {
        None => (cd, None),
        Some((a, b)) => (Cmd::Cd(a.to_string()), Some(Cmd::Cd(b.to_string())))
    }
}

fn test() {
    let r = Cmd::Cd("../dir_a/dir_b/dir_c".to_string());
    dbg!(r);
    assert!(false);
}

pub(crate) fn command(input: &str) -> IResult<&str, Cmd> {
    preceded(
        tag("$ "),
        alt((map(ls, |nodes| Cmd::Ls(nodes)), map(cd, |dir| Cmd::Cd(dir)))),
    )(input)
}
pub(crate) fn terminal(input: &str) -> IResult<&str, Vec<Cmd>> {
    many0(terminated(command, line_ending))(input)
}

#[cfg(test)]
mod test {
    use crate::{
        fs::{Directory, DiskSize, File},
        parser::{cd, dir, dir_node, file, file_node, ls, nodes, path, pathbuf, usize},
        TERM,
    };
    use std::path::PathBuf;

    use super::terminal;

    #[test]
    fn parse_path() {
        let input = "/directory/b.txt";
        assert_eq!(PathBuf::from(input), pathbuf(input).unwrap().1);
    }
    #[test]
    #[should_panic]
    fn parse_path_fails_if_empty() {
        let input = "";
        assert!(path(input).unwrap().1 == ""); // should panic on unwrap
    }
    #[test]
    fn parse_usize() {
        let input = "99";
        assert_eq!(99, usize(input).unwrap().1);
    }
    #[test]
    fn parse_file() {
        let input = "14848514 b.txt";
        assert_eq!(
            (14848514, PathBuf::from("b.txt")),
            file(input).map(|(_, f)| (f.size, f.name)).unwrap()
        );
    }
    #[test]
    fn parse_file_node() {
        let input = "14848514 b.txt";
        assert_eq!(14848514, file_node(input).map(|(_, f)| (f.size())).unwrap());
    }

    #[test]
    fn parse_dir() {
        let input = "dir dirname";
        assert_eq!(Directory::new("dirname").name, dir(input).unwrap().1.name);
    }

    #[test]
    fn parse_dir_node() {
        let input = "dir dirname";
        assert_eq!(0, dir_node(input).map(|(_, f)| (f.size())).unwrap());
    }

    #[test]
    fn parse_nodes() {
        let input = "\n14848514 b.txt\n29116 f\n2557 g\n62596 h.lst\n"; // sum is 14942783
        let files = nodes(input).unwrap().1;
        let mut parent = Directory::new("test");
        for f in files {
            parent.push(f);
        }
        assert_eq!(14942783, parent.size());

        let mut d2 = Directory::new("d2");
        d2.push(File::new("fa", 100)).push(File::new("fb", 1000));
        parent.push(d2);

        assert_eq!(14942783 + 1100, parent.size());
    }

    #[test]
    fn parse_ls() {
        let input = "ls\n14848514 b.txt\n29116 f\n2557 g\n62596 h.lst\n"; // sum is 14942783
        let nodes = ls(input).unwrap().1;
        let size_sum = nodes.iter().fold(0_usize, |sum, n| sum + n.size());
        assert_eq!(size_sum, 14942783);
    }

    #[test]
    fn parse_ls_empty() {
        let input = "ls"; // sum is 14942783
        assert!(ls(input).is_ok());
        assert!(ls(input).unwrap().1.is_empty());
    }

    #[test]
    fn parse_cd() {
        let input = "cd directory_x"; // sum is 14942783
        assert!(cd(input).unwrap().1 == "directory_x");
    }

    #[test]
    fn parse_command() {
        let input = "cd directory_x"; // sum is 14942783
        assert!(cd(input).unwrap().1 == "directory_x");
    }

    #[test]
    fn count_commands_of_parsed_terminal() {
        let cmds = terminal(TERM).unwrap().1;
        cmds.iter().for_each(|n| println!("{}", n));
        assert_eq!(cmds.len(), 10)
    }
}
