use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("./input")?;

    let result_part1_v1 = run_part1_v1(&input)?;
    println!("result_part1_v1: {result_part1_v1}");

    let result_part2_v1 = run_part2_v1(&input)?;
    println!("result_part2_v1: {result_part2_v1}");

    Ok(())
}

fn run_part1_v1(input: &str) -> Result<usize, Box<dyn Error>> {
    let parsed = input.lines().map(|line| {
        let (left, right) = line.split_once(',').unwrap();
        let (start1, end1) = left.split_once('-').unwrap();
        let (start2, end2) = right.split_once('-').unwrap();

        let [start1, end1, start2, end2] =
            [start1, end1, start2, end2].map(|s| s.parse::<i32>().unwrap());

        (start1, end1, start2, end2)
    });

    let result = parsed
        .filter(|(start1, end1, start2, end2)| {
            (start1 <= start2 && end1 >= end2) || (start2 <= start1 && end2 >= end1)
        })
        .count();
    Ok(result)
}

fn run_part2_v1(input: &str) -> Result<usize, Box<dyn Error>> {
    let parsed = input.lines().map(|line| {
        let (left, right) = line.split_once(',').unwrap();
        let (start1, end1) = left.split_once('-').unwrap();
        let (start2, end2) = right.split_once('-').unwrap();

        let [start1, end1, start2, end2] =
            [start1, end1, start2, end2].map(|s| s.parse::<i32>().unwrap());

        (start1, end1, start2, end2)
    });

    let result = parsed
        .filter(|(start1, end1, start2, end2)| (end1 >= start2 && start1 <= end2))
        .count();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    static INPUT_EXAMPLE: &str = include_str!("../input_example");
    static INPUT_EXAMPLE2: &str = include_str!("../input_example2");

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        let result_input_example = run_part1_v1(INPUT_EXAMPLE)?;
        assert_eq!(result_input_example, 2);
        Ok(())
    }

    #[test]
    fn test2_part1() -> Result<(), Box<dyn Error>> {
        let result_input_example = run_part1_v1(INPUT_EXAMPLE2)?;
        assert_eq!(result_input_example, 6);
        Ok(())
    }

    #[test]
    fn test_part2_v1() -> Result<(), Box<dyn Error>> {
        let result_input_example = run_part2_v1(INPUT_EXAMPLE)?;
        assert_eq!(result_input_example, 4);
        Ok(())
    }

    #[test]
    fn test2_part2_v1() -> Result<(), Box<dyn Error>> {
        let result_input_example = run_part2_v1(INPUT_EXAMPLE2)?;
        assert_eq!(result_input_example, 8);
        Ok(())
    }
}
