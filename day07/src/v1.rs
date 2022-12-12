//! V1: We build a graph representing the filetree system and then
//! write a function that traverse the tree, calculating each node
//! size and calling a callback function with the size of the item.
//! This called callback is responsible for having a proper algorithm
//! to determine the result.

use color_eyre::eyre::{eyre, Result};

#[derive(Debug)]
pub enum FsItem {
    //I hope Rust will get someday this: https://github.com/rust-lang/rfcs/pull/2593
    //to avoid this ugly "Name(Name)" syntax
    Dir(Dir),
    File(File),
}

#[derive(Debug)]
pub struct Dir {
    pub name: String,
    pub children: Vec<FsItem>,
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub size: u32,
}

use FsItem::*;

pub fn parse_input_to_root_dir(input: &str) -> Result<Dir> {
    let mut root = Dir {
        name: "/".to_string(),
        children: vec![],
    };

    fn find_dir<'r>(path: &[String], root: &'r mut Dir) -> Option<&'r mut Dir> {
        let mut curr_found_dir = Some(root);
        assert_eq!(path[0], "/");
        for dir in path.iter().skip(1) {
            if let Some(fd) = curr_found_dir {
                curr_found_dir = fd.children.iter_mut().find_map(|c| {
                    if let FsItem::Dir(d) = c {
                        if d.name == *dir {
                            return Some(d);
                        }
                    }
                    None
                });
            }
            if curr_found_dir.is_none() {
                break;
            }
        }
        curr_found_dir
    }

    let mut lines_iter = input.lines().peekable();

    let mut current_path = vec![];

    while let Some(line) = lines_iter.next() {
        if line.starts_with("$ cd /") {
            current_path = vec!["/".to_string()];
        } else if let Some(new_dir) = line.strip_prefix("$ cd ") {
            if new_dir == ".." {
                current_path.pop();
            } else {
                current_path.push(new_dir.to_string());
            }
        } else if line.starts_with("$ ls") {
            //other option was to write: while matches!(lines_iter.peek(), Some(line) if !line.starts_with('$')) {
            //unfortunately Rust doesn;t yet have "if/while let chains": https://rust-lang.github.io/rfcs/2497-if-let-chains.html
            while let Some(line) = lines_iter.peek() {
                // processing only directory listing
                if line.starts_with('$') {
                    //break out of the loop, next iteration will handle the command
                    break;
                }

                let ls_entry_str = lines_iter.next().unwrap(); //safe because we peeked before
                let current_dir = find_dir(&current_path, &mut root);

                if let Some(dir_name) = ls_entry_str.strip_prefix("dir ") {
                    if let Some(cd) = current_dir {
                        cd.children.push(FsItem::Dir(Dir {
                            name: dir_name.to_string(),
                            children: vec![],
                        }));
                    }
                } else {
                    let (size, name) = ls_entry_str.split_once(' ').ok_or(eyre!("wrong input"))?;
                    let size: u32 = size.parse()?;
                    let name: String = name.into();
                    if let Some(cd) = current_dir {
                        cd.children.push(FsItem::File(File { name, size }));
                    }
                }
            }
        }
    }

    Ok(root)
}

// code doing part one for parsed input (written before I wrote code for parsing input string)
pub fn run_part1_v1_parsed_input(root: &Dir) -> Result<String> {
    let size_threshold = 100000;

    let mut result_sum = 0;
    for c in &root.children {
        count_size(c, &mut |count, fs_item| {
            if let Dir(_) = fs_item {
                if count <= size_threshold {
                    result_sum += count;
                }
            }
        });
    }

    Ok(result_sum.to_string())
}

pub fn run_part1_v1(input: &str) -> Result<String> {
    let root = parse_input_to_root_dir(input)?;
    run_part1_v1_parsed_input(&root)
}

pub fn count_size(item: &FsItem, size_count_callback: &mut impl FnMut(u32, &FsItem)) -> u32 {
    match item {
        &File(File { size, .. }) => {
            size_count_callback(size, item);
            size
        }
        Dir(Dir { children, .. }) => {
            let sum = children
                .iter()
                .fold(0, |acc, child| acc + count_size(child, size_count_callback));
            size_count_callback(sum, item);
            sum
        }
    }
}

pub fn run_part2_v1(input: &str) -> Result<String> {
    let file_system_space = 70000000;
    let needed_unused_space = 30000000;

    let root = parse_input_to_root_dir(input)?;

    let root_size = root
        .children
        .iter()
        .map(|c| count_size(c, &mut |_count, _fs_item| {}))
        .sum::<u32>();

    let current_size_of_unused_space = file_system_space - root_size;
    let size_to_delete = needed_unused_space - current_size_of_unused_space;

    let mut smallest_dir_size_to_delete: Option<(String, u32)> = None;
    for c in root.children {
        count_size(&c, &mut |fs_item_size, fs_item| {
            if let Dir(dir_name) = fs_item {
                match smallest_dir_size_to_delete {
                    None => {
                        if fs_item_size >= size_to_delete {
                            smallest_dir_size_to_delete =
                                Some((dir_name.name.clone(), fs_item_size))
                        }
                    }
                    Some((_, current_smallest_dir_size_to_delete)) => {
                        if fs_item_size >= size_to_delete
                            && fs_item_size < current_smallest_dir_size_to_delete
                        {
                            smallest_dir_size_to_delete =
                                Some((dir_name.name.clone(), fs_item_size));
                        }
                    }
                }
            }
        });
    }

    let smallest_dir_size_to_delete = smallest_dir_size_to_delete.ok_or(eyre!("logic error"))?;
    Ok(smallest_dir_size_to_delete.1.to_string())
}
