use color_eyre::eyre::Result;

pub fn run_part1_v1(_input: &str) -> Result<String> {
    Ok("".into())
}

pub fn run_part2_v1(_input: &str) -> Result<String> {
    Ok("".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_EXAMPLE: &str = include_str!("../input_example");

    #[test]
    fn test_part1() -> Result<()> {
        let result_input_example = run_part1_v1(INPUT_EXAMPLE)?;
        assert_eq!(result_input_example, "");
        Ok(())
    }

    #[test]
    fn test_part2_v1() -> Result<()> {
        let result_input_example = run_part2_v1(INPUT_EXAMPLE)?;
        assert_eq!(result_input_example, "");
        Ok(())
    }
}
