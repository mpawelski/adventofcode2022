use color_eyre::eyre::Result;

use super::*;

static INPUT_EXAMPLE: &str = include_str!("../input_example");
static INPUT: &str = include_str!("../input");

#[test]
fn test_part1_v1() -> Result<()> {
    let result = v1::run_part1_v1(INPUT_EXAMPLE)?;
    assert_eq!(result, "21");
    let result = v1::run_part1_v1(INPUT)?;
    assert_eq!(result, "1693");
    Ok(())
}

#[test]
fn test_part1_v2() -> Result<()> {
    let result = v2::run_part1_v2(INPUT_EXAMPLE)?;
    assert_eq!(result, "21");
    let result = v2::run_part1_v2(INPUT)?;
    assert_eq!(result, "1693");
    Ok(())
}

#[test]
fn test_part1_v3() -> Result<()> {
    let result = v3::run_part1_v3(INPUT_EXAMPLE)?;
    assert_eq!(result, "21");
    let result = v3::run_part1_v3(INPUT)?;
    assert_eq!(result, "1693");
    Ok(())
}

#[test]
fn test_part1_v4() -> Result<()> {
    let result = v4::run_part1_v4(INPUT_EXAMPLE)?;
    assert_eq!(result, "21");
    let result = v4::run_part1_v4(INPUT)?;
    assert_eq!(result, "1693");
    Ok(())
}

#[test]
fn test_part1_v5() -> Result<()> {
    let result = v5::run_part1_v5(INPUT_EXAMPLE)?;
    assert_eq!(result, "21");
    let result = v5::run_part1_v5(INPUT)?;
    assert_eq!(result, "1693");
    Ok(())
}

#[test]
fn test_part2_v1() -> Result<()> {
    let result = v1::run_part2_v1(INPUT_EXAMPLE)?;
    assert_eq!(result, "8");
    let result = v1::run_part2_v1(INPUT)?;
    assert_eq!(result, "422059");
    Ok(())
}

#[test]
fn test_part2_v2() -> Result<()> {
    let result = v2::run_part2_v2(INPUT_EXAMPLE)?;
    assert_eq!(result, "8");
    let result = v2::run_part2_v2(INPUT)?;
    assert_eq!(result, "422059");
    Ok(())
}

#[test]
fn test_part2_v3() -> Result<()> {
    let result = v3::run_part2_v3(INPUT_EXAMPLE)?;
    assert_eq!(result, "8");
    let result = v3::run_part2_v3(INPUT)?;
    assert_eq!(result, "422059");
    Ok(())
}

#[test]
fn test_part2_v4() -> Result<()> {
    let result = v4::run_part2_v4(INPUT_EXAMPLE)?;
    assert_eq!(result, "8");
    let result = v4::run_part2_v4(INPUT)?;
    assert_eq!(result, "422059");
    Ok(())
}

#[test]
fn test_part2_v5() -> Result<()> {
    let result = v5::run_part2_v5(INPUT_EXAMPLE)?;
    assert_eq!(result, "8");
    let result = v5::run_part2_v5(INPUT)?;
    assert_eq!(result, "422059");
    Ok(())
}
