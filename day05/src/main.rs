use color_eyre::eyre::Result;
use day05::*;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./input")?;

    let result_part1_v1 = run_part1_v1(&input)?;
    println!("result_part1_v1: {result_part1_v1}");

    let result_part1_v2 = run_part1_v2(&input)?;
    println!("result_part1_v2: {result_part1_v2}");

    let result_part1_v3 = run_part1_v3(&input)?;
    println!("result_part1_v3: {result_part1_v3}");

    let result_part2_v1 = run_part2_v1(&input)?;
    println!("result_part2_v1: {result_part2_v1}");

    let result_part2_v2 = run_part2_v2(&input)?;
    println!("result_part2_v2: {result_part2_v2}");

    Ok(())
}
