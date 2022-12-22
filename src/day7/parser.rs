use std::path::PathBuf;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, digit1, line_ending},
    combinator::{map, map_res},
    multi::many0,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

use crate::fs::{Directory, DiskSize, File, Node};

pub(crate) fn path(input: &str) -> IResult<&str, PathBuf> {
    map(
        take_while1(|c: char| match c as u8 {
            b'a'..=b'z' => true,
            b'A'..=b'Z' => true,
            b'_' | b'-' | b'/' | b'.' => true,
            _ => false,
        }),
        Into::into,
    )(input)
}

#[test]
fn parse_path() {
    let input = "/directory/b.txt";
    assert_eq!(PathBuf::from(input), path(input).unwrap().1);
}

#[test]
#[should_panic]
fn parse_path_fails_if_empty() {
    let input = "";
    assert!(path(input).unwrap().1 == PathBuf::from("")); // should panic on unwrap
}

pub(crate) fn usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |b: &str| b.parse::<usize>())(input)
}

#[test]
fn parse_usize() {
    let input = "99";
    assert_eq!(99, usize(input).unwrap().1);
}

pub(crate) fn file(input: &str) -> IResult<&str, File> {
    map(separated_pair(usize, char(' '), path), |(size, name)| {
        File::new(name, size)
    })(input)
}

#[test]
fn parse_file() {
    let input = "14848514 b.txt";
    assert_eq!(
        (14848514, PathBuf::from("b.txt")),
        file(input).map(|(r_, f)| (f.size, f.name)).unwrap()
    );
}

pub(crate) fn file_node(input: &str) -> IResult<&str, Node> {
    map(file, |file| file.into())(input)
}

#[test]
fn parse_file_node() {
    let input = "14848514 b.txt";
    assert_eq!(14848514, file_node(input).map(|(_, f)| (f.size())).unwrap());
}

pub(crate) fn dir(input: &str) -> IResult<&str, Directory> {
    let parser = separated_pair(tag("dir"), char(' '), path);
    map(parser, |(_, path)| Directory::new(path))(input)
}

#[test]
fn parse_dir() {
    let input = "dir dirname";
    assert_eq!(Directory::new("dirname").name, dir(input).unwrap().1.name);
}

pub(crate) fn dir_node(input: &str) -> IResult<&str, Node> {
    map(dir, |d| d.into())(input)
}

#[test]
fn parse_dir_node() {
    let input = "dir dirname";
    assert_eq!(0, dir_node(input).map(|(_, f)| (f.size())).unwrap());
}

pub(crate) fn nodes(input: &str) -> IResult<&str, Vec<Node>> {
    many0(terminated(alt((file_node, dir_node)), line_ending))(input)
}

#[test]
fn parse_nodes() {
    let input = "14848514 b.txt\n29116 f\n2557 g\n62596 h.lst\n"; // sum is 14942783
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

pub(crate) enum Cmd {
    Ls { ret: Vec<Node> },
    Cd { arg: Directory },
}

pub(crate) fn ls(input: &str) -> IResult<&str, Vec<Node>> {
    preceded(tag("ls\n"), nodes)(input)
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
    let input = "ls\n"; // sum is 14942783
    assert!(ls(input).is_ok());
    assert!(ls(input).unwrap().1.is_empty());
}

pub(crate) fn cd(input: &str) -> IResult<&str, Directory> {
    preceded(tag("cd "), map(path, |pb| Directory::new(pb)))(input)
}

#[test]
fn parse_cd() {
    let input = "cd directory_x"; // sum is 14942783
    assert!(cd(input).unwrap().1.name == PathBuf::from("directory_x"));
}

fn command(input: &str) -> IResult<&str, Cmd> {
    alt((
        map(ls, |nodes| Cmd::Ls { ret: nodes }),
        map(cd, |dir| Cmd::Cd { arg: dir }),
    ))(input)
}
