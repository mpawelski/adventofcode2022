//! V4. Based on https://www.reddit.com/r/adventofcode/comments/zesk40/comment/iz8fww6/?utm_source=share&utm_medium=web2x&context=3
//! Using only hashmaps to build path->size map and later iterate on it to find the result.
//! Mu easier code to read and write, but assumes that we list every file in input only once.
//! Very nice use of pattern matching for matching input lines.

use std::collections::HashMap;

use color_eyre::eyre::{eyre, Result};

fn get_dir_size_map(input: &str) -> Result<HashMap<String, u32>, color_eyre::Report> {
    let mut curr = vec![];
    let mut dirs: HashMap<String, u32> = HashMap::new();
    for line in input.lines() {
        let splited = line.split(' ').collect::<Vec<_>>();
        match splited.as_slice() {
            ["$", "cd", "/"] => curr.push("/".to_string()),
            ["$", "cd", ".."] => {
                curr.pop();
            }
            ["$", "cd", x] => curr.push(x.to_string() + "/"),
            ["$", "ls"] => {}
            ["dir", _] => {}
            [size, _name] => {
                let size = size.parse::<u32>()?;
                curr.iter().fold("".to_string(), |acc, i| {
                    let path = acc + i.as_str();
                    dirs.entry(path.clone())
                        .and_modify(|s| *s += size)
                        .or_insert(size);
                    path
                });
            }
            _ => return Err(eyre!("not supported input")),
        }
    }
    Ok(dirs)
}

pub fn run_part1_v4(input: &str) -> Result<String> {
    let dirs = get_dir_size_map(input)?;

    // dbg!(&dirs);

    let result = dirs.values().filter(|v| **v <= 100_000).sum::<u32>();

    Ok(result.to_string())
}

pub fn run_part2_v4(input: &str) -> Result<String> {
    let file_system_space = 70000000;
    let needed_unused_space = 30000000;

    let dirs = get_dir_size_map(input)?;

    let current_size_of_unused_space = file_system_space - dirs["/"];
    let min_size_to_delete = needed_unused_space - current_size_of_unused_space;

    // dbg!(&dirs);

    let result = dirs
        .values()
        .filter(|v| **v >= min_size_to_delete)
        .min()
        .ok_or(eyre!("no result..."))?;

    Ok(result.to_string())
}
