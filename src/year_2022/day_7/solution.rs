use crate::common::inputs::challenge_test_suite;
use itertools::Itertools;
use std::{collections::VecDeque, iter::Map, str::Split};

#[derive(Debug, Clone)]
enum ParsedFile {
    File { name: String, size: usize },
    Directory(String),
}

#[derive(Debug, Clone)]
enum Command {
    MoveToRoot,
    MoveToParentFolder,
    MoveToFolder(String),
    ListDir(Vec<ParsedFile>),
}

fn get_commands<'a>(str: &'a String) -> impl Iterator<Item = Command> + 'a {
    str.split('$').filter(|x| !x.is_empty()).map(Command::from)
}

impl<'a> From<&str> for Command {
    fn from(str: &str) -> Self {
        let str = str.to_owned();
        let mut raw_strs = str.split_ascii_whitespace().filter(|x| !x.is_empty());
        let cmd_str = raw_strs.next().unwrap();

        match cmd_str {
            "cd" => {
                let dir = raw_strs.next().unwrap();
                match dir {
                    "/" => Command::MoveToRoot,
                    ".." => Command::MoveToParentFolder,
                    _ => Command::MoveToFolder(dir.to_owned()),
                }
            }
            "ls" => {
                let mut files: Vec<ParsedFile> = vec![];
                while let Some(first) = raw_strs.next() {
                    let second = raw_strs.next().unwrap();
                    let parsed_file = if first == "dir" {
                        ParsedFile::Directory(second.to_owned())
                    } else {
                        ParsedFile::File {
                            name: second.to_owned(),
                            size: first.parse().unwrap(),
                        }
                    };
                    files.push(parsed_file);
                }
                Command::ListDir(files)
            }
            _ => unreachable!("Unknown command {}", cmd_str),
        }
    }
}

pub fn solution_1(file_contents: String) -> usize {
    let commands = get_commands(&file_contents).collect_vec();
    println!("{:?}", commands);
    1
}

pub fn solution_2(file_contents: String) -> usize {
    1
}

challenge_test_suite!(
    solution_1,
    10,
    1802,
    solution_2,
    29,
    3551,
    "src",
    "year_2022",
    "day_7"
);
