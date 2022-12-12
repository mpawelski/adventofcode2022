use color_eyre::eyre::Result;
use day07::*;

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = std::fs::read_to_string("./input")?;

    let result_part1_v1 = v1::run_part1_v1(&input)?;
    println!("result_part1_v1: {result_part1_v1}");

    let result_part2_v1 = v1::run_part2_v1(&input)?;
    println!("result_part2_v1: {result_part2_v1}");

    let result_part2_v2 = v2::run_part2_v2(&input)?;
    println!("result_part2_v2: {result_part2_v2}");

    let result_part2_v2_smallstring = v2_smartstring::run_part2_v2_smartstring(&input)?;
    println!("result_part2_v2_smallstring: {result_part2_v2_smallstring}");

    let result_part2_v3 = v3::run_part2_v3(&input)?;
    println!("result_part2_v3: {result_part2_v3}");

    let result_part2_v4 = v4::run_part2_v4(&input)?;
    println!("result_part2_v4: {result_part2_v4}");

    let result_part2_v5 = v5::run_part2_v5(&input)?;
    println!("result_part2_v5: {result_part2_v5}");

    Ok(())
}
