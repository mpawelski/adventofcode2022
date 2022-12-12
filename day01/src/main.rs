use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("./input")?;

    let result_part1 = run_part1_v1(&input)?;
    println!("run_part1_v1(input): {result_part1}");

    let result_part2 = run_part2_v1(&input)?;
    println!("run_part1_v1(input): {result_part2}");

    Ok(())
}

fn run_part1_v1(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut sums = vec![];
    let mut tmp = 0;
    for line in input.lines() {
        if line.is_empty() {
            tmp = 0;
            continue;
        }
        let num_parsed: i32 = line.parse()?;
        tmp += num_parsed;
        sums.push(tmp)
    }

    let max_value = sums
        .into_iter()
        .max()
        .ok_or("Sum array is empty. Probably because of empty input")?;
    Ok(max_value)
}

fn run_part2_v1(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut sums = vec![];
    let mut tmp = 0;
    for line in input.lines() {
        if line.is_empty() {
            sums.push(tmp);
            tmp = 0;
            continue;
        }
        let num_parsed: i32 = line.parse()?;
        tmp += num_parsed;
    }
    sums.push(tmp);

    sums.sort_by(|a, b| b.cmp(a)); // sort descending order
    let max_value = sums.iter().take(3).sum::<i32>();
    Ok(max_value)
}

//TODO: write V2, with iterator usage and take_while()

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    static INPUT_EXAMPLE: &str = include_str!("../input_example");

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        let result_input_example = run_part1_v1(INPUT_EXAMPLE)?;
        assert_eq!(result_input_example, 24000);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Box<dyn Error>> {
        let result_input_example = run_part2_v1(INPUT_EXAMPLE)?;
        assert_eq!(result_input_example, 45000);
        Ok(())
    }
}
