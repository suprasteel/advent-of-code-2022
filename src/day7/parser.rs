use std::path::PathBuf;

use nom::{
    bytes::complete::{tag, take_while},
    character::complete::{char, digit1, line_ending},
    combinator::{map, map_res},
    multi::many0,
    sequence::{separated_pair, terminated},
    IResult,
};

use crate::fs::{Directory, File, Node};

fn path(input: &str) -> IResult<&str, PathBuf> {
    map(
        take_while(|c: char| match c as u8 {
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

fn usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |b: &str| b.parse::<usize>())(input)
}

#[test]
fn parse_usize() {
    let input = "99";
    assert_eq!(99, usize(input).unwrap().1);
}

fn file(input: &str) -> IResult<&str, File> {
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

fn dir(input: &str) -> IResult<&str, Directory> {
    let parser = separated_pair(tag("dir"), char(' '), path);
    map(parser, |(_, path)| Directory::new(path))(input)
}

#[test]
fn parse_dir() {
    let input = "dir dirname";
    assert_eq!(Directory::new("dirname").name, dir(input).unwrap().1.name);
}

fn dir_or_file(input: &str) -> IResult<&str, Vec<Node>> {
    
}

// -> Change to node
fn files_with_size(input: &str) -> IResult<&str, Vec<File>> {
    many0(terminated(file, line_ending))(input)
}

#[test]
fn parse_files_with_size() {
    let input = "14848514 b.txt\n29116 f\n2557 g\n62596 h.lst\n";
    assert_eq!(
        vec![
            (14848514, PathBuf::from("b.txt")),
            (29116, "f".into()),
            (2557, "g".into()),
            (62596, "h.lst".into())
        ],
        files_with_size(input).unwrap().1
    );
}

enum Cmd {
    Ls { files: (usize, PathBuf) },
    Cd { path: PathBuf },
}

fn cmd(input: &str) -> IResult<&str, Cmd> {}

fn parse_ls(i: &str) -> IResult<&str, Cmd> {
    map(tag("ls"), |_| Ls)(i)
}

fn parse_ls_out(i: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(i)
}

/*
#[derive(Debug)]
struct Cd(Utf8PathBuf);

fn parse_cd(i: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(i)
}

#[derive(Debug)]
enum Command {
    Ls(Ls),
    Cd(Cd),
}

fn parse_cmd(i: &str) -> IResult<&str, Command> {
let (i, _) = tag("$ ")(i)?;
alt((map(parse_ls, Command::Ls), map(parse_cd, Command::Cd)))(i)
}
*/
