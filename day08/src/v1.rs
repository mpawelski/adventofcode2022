use color_eyre::eyre::Result;
use to_vec::*;

pub fn run_part1_v1(input: &str) -> Result<String> {
    let parsed_area = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).to_vec())
        .to_vec();
    let width = parsed_area[0].len();
    let height = parsed_area.len();
    let mut visible_count = 0;
    for i in 1..height - 1 {
        for j in 1..width - 1 {
            let curr_value = parsed_area[i][j];
            let mut is_visible = false;

            let mut other_trees_check_count = 0;
            let mut smaller_trees_count = 0;
            for x in 0..i {
                other_trees_check_count += 1;
                if parsed_area[x][j] < curr_value {
                    smaller_trees_count += 1;
                }
            }
            if smaller_trees_count == other_trees_check_count {
                is_visible = true;
            }

            let mut other_trees_check_count = 0;
            let mut smaller_trees_count = 0;
            for x in i + 1..height {
                other_trees_check_count += 1;
                if parsed_area[x][j] < curr_value {
                    smaller_trees_count += 1;
                }
            }
            if smaller_trees_count == other_trees_check_count {
                is_visible = true;
            }

            let mut other_trees_check_count = 0;
            let mut smaller_trees_count = 0;
            for x in 0..j {
                other_trees_check_count += 1;
                if parsed_area[i][x] < curr_value {
                    smaller_trees_count += 1;
                }
            }
            if smaller_trees_count == other_trees_check_count {
                is_visible = true;
            }

            let mut other_trees_check_count = 0;
            let mut smaller_trees_count = 0;
            for x in j + 1..width {
                other_trees_check_count += 1;
                if parsed_area[i][x] < curr_value {
                    smaller_trees_count += 1;
                }
            }
            if smaller_trees_count == other_trees_check_count {
                is_visible = true;
            }

            if is_visible {
                visible_count += 1;
            }
        }
    }

    let visible_with_edges = visible_count + width * 2 + (height - 2) * 2;

    Ok(visible_with_edges.to_string())
}

#[allow(clippy::comparison_chain)]
pub fn run_part2_v1(input: &str) -> Result<String> {
    let parsed_area = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).to_vec())
        .to_vec();
    let width = parsed_area[0].len();
    let height = parsed_area.len();

    let mut current_best_result = 0;

    for i in 1..height - 1 {
        for j in 1..width - 1 {
            let curr_value = parsed_area[i][j];
            let mut result = 1;

            let mut smaller_trees_count = 0;
            for x in (0..i).rev() {
                if parsed_area[x][j] < curr_value {
                    smaller_trees_count += 1;
                } else {
                    smaller_trees_count += 1;
                    break;
                }
            }

            result *= smaller_trees_count;

            let mut smaller_trees_count = 0;
            for x in i + 1..height {
                if parsed_area[x][j] < curr_value {
                    smaller_trees_count += 1;
                } else {
                    smaller_trees_count += 1;
                    break;
                }
            }
            result *= smaller_trees_count;

            let mut smaller_trees_count = 0;
            for x in (0..j).rev() {
                if parsed_area[i][x] < curr_value {
                    smaller_trees_count += 1;
                } else {
                    smaller_trees_count += 1;
                    break;
                }
            }
            result *= smaller_trees_count;

            let mut smaller_trees_count = 0;
            for x in j + 1..width {
                if parsed_area[i][x] < curr_value {
                    smaller_trees_count += 1;
                } else {
                    smaller_trees_count += 1;
                    break;
                }
            }
            result *= smaller_trees_count;

            if result > current_best_result {
                current_best_result = result;
            }
        }
    }

    Ok(current_best_result.to_string())
}
