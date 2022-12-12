//! V2 but uses SmallString crate.
//!
//!
use color_eyre::eyre::{eyre, Result};

use smartstring::alias::String;

#[derive(Debug)]
enum FsItem {
    //I hope Rust will get someday this: https://github.com/rust-lang/rfcs/pull/2593
    //to avoid this ugly "Name(Name)" syntax
    Dir(Dir),
    File(File),
}
impl FsItem {
    fn size(&self) -> u32 {
        match self {
            Dir(d) => d.size,
            File(f) => f.size,
        }
    }
}

#[derive(Debug)]
struct Dir {
    name: String,
    children: Vec<FsItem>,
    size: u32,
}

#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}

use FsItem::*;

fn parse_input_to_root_dir(input: &str) -> Result<Dir> {
    let mut root = Dir {
        name: "/".into(),
        children: vec![],
        size: 0,
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

    let mut current_path: Vec<String> = vec![];

    while let Some(line) = lines_iter.next() {
        if line.starts_with("$ cd /") {
            current_path = vec!["/".into()];
        } else if let Some(new_dir) = line.strip_prefix("$ cd ") {
            if new_dir == ".." {
                current_path.pop();
            } else {
                current_path.push(new_dir.into());
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
                            name: dir_name.into(),
                            children: vec![],
                            size: 0,
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

fn count_and_fill_sizes(item: &mut FsItem) -> u32 {
    let item_size = match item {
        File(file) => file.size,
        Dir(dir) => {
            let sum = dir
                .children
                .iter_mut()
                .map(count_and_fill_sizes)
                .sum::<u32>();
            dir.size = sum;

            dir.size
        }
    };

    item_size
}

fn traverse_fs_tree(item: &FsItem, callback: &mut impl FnMut(&FsItem) -> bool) {
    match item {
        File(_file) => {
            callback(item);
        }
        Dir(dir) => {
            let skip_traversing_children = callback(item);
            if !skip_traversing_children {
                for child in &dir.children {
                    traverse_fs_tree(child, callback);
                }
            }
        }
    };
}

// This version first fills up all calculated sized of a directory and tnen traverse the tree to find the dir to delete
pub fn run_part2_v2_smartstring(input: &str) -> Result<std::string::String> {
    let file_system_space = 70000000;
    let needed_unused_space = 30000000;

    let root = parse_input_to_root_dir(input)?;

    let root_fs_item = &mut FsItem::Dir(root);
    count_and_fill_sizes(root_fs_item);

    let current_size_of_unused_space = file_system_space - root_fs_item.size();
    let min_size_to_delete = needed_unused_space - current_size_of_unused_space;

    let mut smallest_dir_to_delete: Option<(String, u32)> = None;

    traverse_fs_tree(root_fs_item, &mut |fs_item| {
        let fs_item_size = fs_item.size();
        if let Dir(dir) = fs_item {
            match smallest_dir_to_delete {
                None => {
                    if fs_item_size >= min_size_to_delete {
                        smallest_dir_to_delete = Some((dir.name.clone(), fs_item_size));
                    }
                }
                Some((_, current_smallest_dir_size_to_delete)) => {
                    if current_smallest_dir_size_to_delete < dir.size {
                        return true;
                    }

                    if fs_item_size >= min_size_to_delete
                        && fs_item_size < current_smallest_dir_size_to_delete
                    {
                        smallest_dir_to_delete = Some((dir.name.clone(), fs_item_size));
                    }
                }
            }
        }
        false
    });

    let smallest_dir_size_to_delete = smallest_dir_to_delete.ok_or(eyre!("logic error"))?;
    Ok(smallest_dir_size_to_delete.1.to_string())
}
