use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("./input")?;

    let result_part1 = run_part1_v1(&input)?;
    println!("run_part1_v1: {result_part1}");

    let result_part1 = run_part2_v1(&input)?;
    println!("run_part2_v1: {result_part1}");

    Ok(())
}

fn run_part1_v1(input: &str) -> Result<i32, Box<dyn Error>> {
    enum RPS {
        Rock,
        Paper,
        Scissors,
    }
    impl RPS {
        fn score(&self) -> i32 {
            match self {
                RPS::Rock => 1,
                RPS::Paper => 2,
                RPS::Scissors => 3,
            }
        }
    }

    let lose_score = 0;
    let draw_score = 3;
    let win_score = 6;

    let mut round_scores = vec![];
    for line in input.lines() {
        let s = line.split(" ").collect::<Vec<_>>();
        let &[opponent, you] = s.as_slice() else { panic!("wrong input!"); };

        let opponent = match opponent {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissors,
            _ => panic!("wrong input"),
        };

        let you = match you {
            "X" => RPS::Rock,
            "Y" => RPS::Paper,
            "Z" => RPS::Scissors,
            _ => panic!("wrong input"),
        };

        let round_score = match (opponent, &you) {
            (RPS::Rock, RPS::Rock) => draw_score + you.score(),
            (RPS::Rock, RPS::Paper) => win_score + you.score(),
            (RPS::Rock, RPS::Scissors) => lose_score + you.score(),
            (RPS::Paper, RPS::Paper) => draw_score + you.score(),
            (RPS::Paper, RPS::Scissors) => win_score + you.score(),
            (RPS::Paper, RPS::Rock) => lose_score + you.score(),
            (RPS::Scissors, RPS::Scissors) => draw_score + you.score(),
            (RPS::Scissors, RPS::Rock) => win_score + you.score(),
            (RPS::Scissors, RPS::Paper) => lose_score + you.score(),
        };

        round_scores.push(round_score);
    }

    let sum_result: i32 = round_scores.iter().sum();
    Ok(sum_result)
}

fn run_part2_v1(input: &str) -> Result<i32, Box<dyn Error>> {
    enum Rps {
        Rock,
        Paper,
        Scissors,
    }
    impl Rps {
        fn score(&self) -> i32 {
            match self {
                Rps::Rock => 1,
                Rps::Paper => 2,
                Rps::Scissors => 3,
            }
        }
    }

    enum RpsResult {
        Win,
        Draw,
        Lose,
    }

    let lose_score = 0;
    let draw_score = 3;
    let win_score = 6;

    let mut round_scores = vec![];
    for line in input.lines() {
        let s = line.split(" ").collect::<Vec<_>>();
        let &[opponent, you] = s.as_slice() else { panic!("wrong input!"); };

        let ref opponent = match opponent {
            "A" => Rps::Rock,
            "B" => Rps::Paper,
            "C" => Rps::Scissors,
            _ => panic!("wrong input"),
        };

        let result = match you {
            "X" => RpsResult::Lose,
            "Y" => RpsResult::Draw,
            "Z" => RpsResult::Win,
            _ => panic!("wrong input"),
        };

        let ref you = match (opponent, result) {
            (Rps::Paper, RpsResult::Draw) => Rps::Paper,
            (Rps::Paper, RpsResult::Win) => Rps::Scissors,
            (Rps::Paper, RpsResult::Lose) => Rps::Rock,
            (Rps::Rock, RpsResult::Draw) => Rps::Rock,
            (Rps::Rock, RpsResult::Win) => Rps::Paper,
            (Rps::Rock, RpsResult::Lose) => Rps::Scissors,
            (Rps::Scissors, RpsResult::Draw) => Rps::Scissors,
            (Rps::Scissors, RpsResult::Win) => Rps::Rock,
            (Rps::Scissors, RpsResult::Lose) => Rps::Paper,
        };

        let round_score = match (opponent, you) {
            (Rps::Rock, Rps::Rock) => draw_score + you.score(),
            (Rps::Rock, Rps::Paper) => win_score + you.score(),
            (Rps::Rock, Rps::Scissors) => lose_score + you.score(),
            (Rps::Paper, Rps::Paper) => draw_score + you.score(),
            (Rps::Paper, Rps::Scissors) => win_score + you.score(),
            (Rps::Paper, Rps::Rock) => lose_score + you.score(),
            (Rps::Scissors, Rps::Scissors) => draw_score + you.score(),
            (Rps::Scissors, Rps::Rock) => win_score + you.score(),
            (Rps::Scissors, Rps::Paper) => lose_score + you.score(),
        };

        round_scores.push(round_score);
    }

    let sum_result: i32 = round_scores.iter().sum();
    Ok(sum_result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    static INPUT_EXAMPLE: &str = include_str!("../input_example");

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        let result_input_example = run_part1_v1(INPUT_EXAMPLE)?;
        assert_eq!(result_input_example, 15);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Box<dyn Error>> {
        let result_input_example = run_part2_v1(INPUT_EXAMPLE)?;
        assert_eq!(result_input_example, 12);
        Ok(())
    }
}
