use std::{collections::HashSet, error::Error, os::windows::prelude::HandleOrInvalid};

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("./input")?;

    let result_part1_v1 = run_part1_v1(&input)?;
    println!("result_part1_v1: {result_part1_v1}");

    let result_part2_v1 = run_part2_v1(&input)?;
    println!("result_part2_v1: {result_part2_v1}");

    let result_part2_v2 = run_part2_v2(&input)?;
    println!("result_part2_v2: {result_part2_v2}");

    Ok(())
}

fn run_part1_v1(input: &str) -> Result<i32, Box<dyn Error>> {
    let result = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);

            let left_hash_set = left.chars().collect::<HashSet<_>>();
            let right_hash_set = right.chars().collect::<HashSet<_>>();
            let elements_in_both = left_hash_set
                .intersection(&right_hash_set)
                .collect::<Vec<_>>();

            let [element] = elements_in_both.as_slice() else {panic!("wrong input!");};

            let priority = match element {
                'a'..='z' => (**element as u8) - b'a' + 1,
                'A'..='Z' => (**element as u8) - b'A' + 27,
                _ => panic!("wrong input! should be just letters"),
            };

            priority as i32
        })
        .sum::<i32>();

    Ok(result)
}

fn run_part2_v1(input: &str) -> Result<i32, Box<dyn Error>> {
    let result = input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|group| {
            let element = group
                .iter()
                .map(|line| line.chars().collect::<HashSet<_>>())
                .reduce(|acc, item| acc.intersection(&item).copied().collect::<HashSet<_>>())
                .unwrap()
                .into_iter()
                .next()
                .unwrap();

            let priority = match element {
                'a'..='z' => (element as u8) - b'a' + 1,
                'A'..='Z' => (element as u8) - b'A' + 27,
                _ => panic!("wrong input! should be just letters"),
            };

            priority as i32
        })
        .sum::<i32>();

    Ok(result)
}

fn run_part2_v2(input: &str) -> Result<i32, Box<dyn Error>> {
    let result = input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|group| {
            let &[a,b,c] = group else {panic!("wrong input")};

            let a = a.chars().collect::<HashSet<_>>();
            let b = b.chars().collect::<HashSet<_>>();
            let c = c.chars().collect::<HashSet<_>>();

            let common_elements = a.intersection(&b).copied().collect::<HashSet<_>>();
            let common_elements = common_elements.intersection(&c).collect::<HashSet<_>>();
            let element = common_elements.iter().next().unwrap();

            let priority = match **element {
                e @ 'a'..='z' => (e as u8) - b'a' + 1,
                e @ 'A'..='Z' => (e as u8) - b'A' + 27,
                _ => panic!("wrong input! should be just letters"),
            };

            priority as i32
        })
        .sum::<i32>();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    static INPUT_EXAMPLE: &str = include_str!("../input_example");

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        let result_input_example = run_part1_v1(INPUT_EXAMPLE)?;
        assert_eq!(result_input_example, 157);
        Ok(())
    }

    #[test]
    fn test_part2_v1() -> Result<(), Box<dyn Error>> {
        let result_input_example = run_part2_v1(INPUT_EXAMPLE)?;
        assert_eq!(result_input_example, 70);
        Ok(())
    }

    #[test]
    fn test_part2_v2() -> Result<(), Box<dyn Error>> {
        let result_input_example = run_part2_v2(INPUT_EXAMPLE)?;
        assert_eq!(result_input_example, 70);
        Ok(())
    }
}
