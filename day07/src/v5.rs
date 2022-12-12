//! V5. Same as V4 but use BTreeMap instead of HashMap.

use std::collections::BTreeMap;

use color_eyre::eyre::{eyre, Result};
fn get_dir_size_map(input: &str) -> Result<BTreeMap<String, u32>, color_eyre::Report> {
    let mut curr = vec![];
    let mut dirs: BTreeMap<String, u32> = BTreeMap::new();
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

pub fn run_part1_v5(input: &str) -> Result<String> {
    let dirs = get_dir_size_map(input)?;

    let result = dirs.values().filter(|v| **v <= 100_000).sum::<u32>();

    Ok(result.to_string())
}

pub fn run_part2_v5(input: &str) -> Result<String> {
    let file_system_space = 70000000;
    let needed_unused_space = 30000000;

    let dirs = get_dir_size_map(input)?;
    // dbg!(&dirs);

    let current_size_of_unused_space = file_system_space - dirs["/"];
    let min_size_to_delete = needed_unused_space - current_size_of_unused_space;

    let result = dirs
        .values()
        .filter(|v| **v >= min_size_to_delete)
        .min()
        .ok_or(eyre!("no result..."))?;

    Ok(result.to_string())
}
