use crate::common::inputs::challenge_test_suite;
use itertools::Itertools;
use std::{
    collections::{HashMap, VecDeque},
    iter::Map,
    str::Split,
};

#[derive(Debug, Clone)]
enum ParsedFile {
    File { name: String, size: usize },
    Directory(String),
}

#[derive(Debug, Clone)]
enum FileTree {
    File { name: String, size: usize },
    Directory { name: String, files: Vec<FileTree> },
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

fn build_file_tree(map: &HashMap<String, Vec<ParsedFile>>, current_path: String) -> FileTree {
    println!("{}", current_path);
    let cur_dir = map.get(&current_path).unwrap();
    let files = cur_dir
        .into_iter()
        .map(|x| -> FileTree {
            match x {
                ParsedFile::File { name, size } => FileTree::File {
                    name: name.clone(),
                    size: size.clone(),
                },
                ParsedFile::Directory(x) => build_file_tree(map, format!("{}{}/", current_path, x)),
            }
        })
        .collect_vec();
    FileTree::Directory {
        name: current_path,
        files,
    }
}

pub fn solution_1(file_contents: String) -> usize {
    let commands = get_commands(&file_contents).collect_vec();
    // path.push("".to_owned());
    // path.push("a".to_owned());
    // path.push("c".to_owned());
    // println!("{:?}", path.join("/"));
    // path.pop();
    // println!("{:?}", path.join("/"));
    let path_map = {
        let mut path: Vec<String> = vec![];
        let mut path_map = HashMap::new();
        commands.iter().for_each(|command| match command {
            Command::MoveToRoot => {
                path.clear();
                path.push("".to_owned());
            }
            Command::MoveToParentFolder => {
                path.pop();
            }
            Command::MoveToFolder(dir) => {
                path.push(dir.to_owned());
            }
            Command::ListDir(contents) => {
                let path = format!("{}/", path.join("/"));
                path_map.insert(path, contents.clone());
            }
        });
        path_map
    };

    println!("{:?}", path_map);
    let file_tree = build_file_tree(&path_map, "/".to_owned());

    // println!("{:?}", commands);
    println!("{:?}", file_tree);
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
