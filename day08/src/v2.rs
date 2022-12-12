use color_eyre::eyre::Result;
use to_vec::*;

pub fn run_part1_v2(input: &str) -> Result<String> {
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

            let mut check_and_set_is_visible =
                |trees_range: std::ops::Range<usize>, get_tree_func: &dyn Fn(usize) -> u32| {
                    let mut other_trees_check_count = 0;
                    let mut smaller_trees_count = 0;
                    for x in trees_range {
                        other_trees_check_count += 1;
                        let tree = get_tree_func(x);
                        if tree < curr_value {
                            smaller_trees_count += 1;
                        }
                    }
                    if smaller_trees_count == other_trees_check_count {
                        is_visible = true;
                    }
                };

            check_and_set_is_visible(0..i, &|x| parsed_area[x][j]);
            check_and_set_is_visible(i + 1..height, &|x| parsed_area[x][j]);
            check_and_set_is_visible(0..j, &|x| parsed_area[i][x]);
            check_and_set_is_visible(j + 1..width, &|x| parsed_area[i][x]);

            if is_visible {
                visible_count += 1;
            }
        }
    }

    let visible_with_edges = visible_count + width * 2 + (height - 2) * 2;

    Ok(visible_with_edges.to_string())
}

#[allow(clippy::comparison_chain)]
pub fn run_part2_v2(input: &str) -> Result<String> {
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

            let mut calculate_result =
                |trees_range: &mut dyn Iterator<Item = usize>,
                 get_tree_func: &dyn Fn(usize) -> u32| {
                    let mut smaller_trees_count = 0;
                    for x in trees_range {
                        let tree = get_tree_func(x);
                        if tree < curr_value {
                            smaller_trees_count += 1;
                        } else {
                            smaller_trees_count += 1;
                            break;
                        }
                    }
                    result *= smaller_trees_count;
                };

            calculate_result(&mut (0..i).rev(), &|x| parsed_area[x][j]);
            calculate_result(&mut (i + 1..height), &|x| parsed_area[x][j]);
            calculate_result(&mut (0..j).rev(), &|x| parsed_area[i][x]);
            calculate_result(&mut (j + 1..width), &|x| parsed_area[i][x]);

            if result > current_best_result {
                current_best_result = result;
            }
        }
    }

    Ok(current_best_result.to_string())
}
