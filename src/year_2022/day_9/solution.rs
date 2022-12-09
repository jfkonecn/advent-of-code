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

fn build_file_tree_rec(map: &HashMap<String, Vec<ParsedFile>>, current_path: String) -> FileTree {
    let cur_dir = map.get(&current_path).unwrap();
    let files = cur_dir
        .into_iter()
        .map(|x| -> FileTree {
            match x {
                ParsedFile::File { name, size } => FileTree::File {
                    name: name.clone(),
                    size: size.clone(),
                },
                ParsedFile::Directory(x) => {
                    build_file_tree_rec(map, format!("{}{}/", current_path, x))
                }
            }
        })
        .collect_vec();
    FileTree::Directory {
        name: current_path,
        files,
    }
}
fn build_file_tree(file_contents: String) -> FileTree {
    let commands = get_commands(&file_contents).collect_vec();
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

    build_file_tree_rec(&path_map, "/".to_owned())
}

fn get_dir_sizes_rec(
    file_trees: &Vec<FileTree>,
    dir_name: String,
) -> (Vec<(String, usize)>, usize) {
    let mut vec: Vec<(String, usize)> = vec![];
    let mut total: usize = 0;
    file_trees.iter().for_each(|file_tree| {
        match file_tree {
            FileTree::File { size, name } => {
                total += size;
            }
            FileTree::Directory { name, files } => {
                let (mut sizes, sum) = get_dir_sizes_rec(files, name.clone());
                total += sum;
                vec.append(&mut sizes);
            }
        };
    });
    vec.push((dir_name, total));
    (vec, total)
}
fn get_dir_sizes<'a>(file_tree: FileTree) -> Vec<(String, usize)> {
    match file_tree {
        FileTree::Directory { name, files } => {
            let (vec, _) = get_dir_sizes_rec(&files, name);
            vec
        }
        _ => panic!("unsupported"),
    }
}

pub fn solution_1(file_contents: String) -> usize {
    let file_tree = build_file_tree(file_contents);
    let sizes = get_dir_sizes(file_tree);
    sizes
        .iter()
        .map(|(_, x)| *x)
        .filter(|x| x.clone() <= 100000usize)
        .sum()
}

pub fn solution_2(file_contents: String) -> usize {
    let file_tree = build_file_tree(file_contents);
    let sizes = get_dir_sizes(file_tree);
    let sizes = sizes.iter().map(|(_, x)| *x).collect_vec();
    let space_taken = sizes.iter().max().unwrap();
    let space_left = 70000000 - space_taken;
    let min_to_delete = 30000000 - space_left;
    sizes
        .iter()
        .map(|x| x.clone())
        .filter(|x| (*x).clone() >= min_to_delete)
        .min()
        .unwrap()
}

challenge_test_suite!(
    solution_1,
    1,
    1,
    solution_2,
    1,
    1,
    "src",
    "year_2022",
    "day_9"
);
