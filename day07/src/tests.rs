use crate::*;

use v1::*;
use v2::*;
use v2_smartstring::*;
use v3::*;
use v4::*;
use v5::*;

fn get_test_root() -> Dir {
    Dir {
        name: "/".to_string(),
        children: vec![
            FsItem::Dir(Dir {
                name: "a".to_string(),
                children: vec![
                    FsItem::Dir(Dir {
                        name: "e".to_string(),
                        children: vec![FsItem::File(File {
                            name: "i".to_string(),
                            size: 584,
                        })],
                    }),
                    FsItem::File(File {
                        name: "f".to_string(),
                        size: 29116,
                    }),
                    FsItem::File(File {
                        name: "g".to_string(),
                        size: 2557,
                    }),
                    FsItem::File(File {
                        name: "h.lst".to_string(),
                        size: 62596,
                    }),
                ],
            }),
            FsItem::File(File {
                name: "b.txt".to_string(),
                size: 14848514,
            }),
            FsItem::File(File {
                name: "c.dat".to_string(),
                size: 8504156,
            }),
            FsItem::Dir(Dir {
                name: "d".to_string(),
                children: vec![
                    FsItem::File(File {
                        name: "j".to_string(),
                        size: 4060174,
                    }),
                    FsItem::File(File {
                        name: "d.log".to_string(),
                        size: 8033020,
                    }),
                    FsItem::File(File {
                        name: "d.ext".to_string(),
                        size: 5626152,
                    }),
                    FsItem::File(File {
                        name: "k".to_string(),
                        size: 7214296,
                    }),
                ],
            }),
        ],
    }
}

use std::{error::Error, sync::Once};

static INPUT_EXAMPLE: &str = include_str!("../input_example");

static INIT: Once = Once::new();

pub fn initialize() {
    INIT.call_once(|| {
        color_eyre::install().unwrap(); // I like colors
    });
}

#[test]
fn test_part1_v1_parsed_input() -> Result<(), Box<dyn Error>> {
    initialize();
    let root = get_test_root();
    let result_input_example = run_part1_v1_parsed_input(&root)?;
    assert_eq!(result_input_example, "95437");
    Ok(())
}

#[test]
fn test_part1() -> Result<(), Box<dyn Error>> {
    initialize();
    let result_input_example = run_part1_v1(INPUT_EXAMPLE)?;
    assert_eq!(result_input_example, "95437");
    Ok(())
}

#[test]
fn test_part2_v1() -> Result<(), Box<dyn Error>> {
    initialize();
    let result_input_example = run_part2_v1(INPUT_EXAMPLE)?;
    assert_eq!(result_input_example, "24933642");
    Ok(())
}

#[test]
fn test_part2_v2() -> Result<(), Box<dyn Error>> {
    initialize();
    let result_input_example = run_part2_v2(INPUT_EXAMPLE)?;
    assert_eq!(result_input_example, "24933642");
    Ok(())
}

#[test]
fn test_part2_v2_smartstring() -> Result<(), Box<dyn Error>> {
    initialize();
    let result_input_example = run_part2_v2_smartstring(INPUT_EXAMPLE)?;
    assert_eq!(result_input_example, "24933642");
    Ok(())
}

#[test]
fn test_part2_v3() -> Result<(), Box<dyn Error>> {
    initialize();
    let result_input_example = run_part2_v3(INPUT_EXAMPLE)?;
    assert_eq!(result_input_example, "24933642");
    Ok(())
}

#[test]
fn test_part1_v4() -> Result<(), Box<dyn Error>> {
    initialize();
    let result_input_example = run_part1_v4(INPUT_EXAMPLE)?;
    assert_eq!(result_input_example, "95437");
    Ok(())
}

#[test]
fn test_part2_v4() -> Result<(), Box<dyn Error>> {
    initialize();
    let result_input_example = run_part2_v4(INPUT_EXAMPLE)?;
    assert_eq!(result_input_example, "24933642");
    Ok(())
}

#[test]
fn test_part1_v5() -> Result<(), Box<dyn Error>> {
    initialize();
    let result_input_example = run_part1_v5(INPUT_EXAMPLE)?;
    assert_eq!(result_input_example, "95437");
    Ok(())
}

#[test]
fn test_part2_v5() -> Result<(), Box<dyn Error>> {
    initialize();
    let result_input_example = run_part2_v5(INPUT_EXAMPLE)?;
    assert_eq!(result_input_example, "24933642");
    Ok(())
}
