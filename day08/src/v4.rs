// Need to use Rust Nightly version. Turn it on in console:
// rustup default nightly

use color_eyre::eyre::Result;
use to_vec::ToVec;

// v3. Using generators (requires nightly rust)
pub fn run_part1_v4(input: &str) -> Result<String> {
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

            let mut check_and_set_is_visible = |trees: &mut dyn Iterator<Item = u32>| {
                let mut other_trees_check_count = 0;
                let mut smaller_trees_count = 0;
                for tree in trees {
                    other_trees_check_count += 1;
                    if tree < curr_value {
                        smaller_trees_count += 1;
                    }
                }
                if smaller_trees_count == other_trees_check_count {
                    is_visible = true;
                }
            };

            check_and_set_is_visible(&mut std::iter::from_generator(|| {
                for x in 0..i {
                    yield parsed_area[x][j];
                }
            }));

            check_and_set_is_visible(&mut std::iter::from_generator(|| {
                for x in i + 1..height {
                    yield parsed_area[x][j];
                }
            }));

            check_and_set_is_visible(&mut std::iter::from_generator(|| {
                for x in 0..j {
                    yield parsed_area[i][x];
                }
            }));

            check_and_set_is_visible(&mut std::iter::from_generator(|| {
                for x in j + 1..width {
                    yield parsed_area[i][x];
                }
            }));

            if is_visible {
                visible_count += 1;
            }
        }
    }

    let visible_with_edges = visible_count + width * 2 + (height - 2) * 2;

    Ok(visible_with_edges.to_string())
}

#[allow(clippy::comparison_chain)]
pub fn run_part2_v4(input: &str) -> Result<String> {
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

            let mut calculate_result = |trees: &mut dyn Iterator<Item = u32>| {
                let mut smaller_trees_count = 0;
                for tree in trees {
                    if tree < curr_value {
                        smaller_trees_count += 1;
                    } else {
                        smaller_trees_count += 1;
                        break;
                    }
                }
                result *= smaller_trees_count;
            };

            calculate_result(&mut std::iter::from_generator(|| {
                for x in (0..i).rev() {
                    yield parsed_area[x][j];
                }
            }));
            calculate_result(&mut std::iter::from_generator(|| {
                for x in i + 1..height {
                    yield parsed_area[x][j];
                }
            }));
            calculate_result(&mut std::iter::from_generator(|| {
                for x in (0..j).rev() {
                    yield parsed_area[i][x];
                }
            }));
            calculate_result(&mut std::iter::from_generator(|| {
                for x in (j + 1..width) {
                    yield parsed_area[i][x];
                }
            }));

            if result > current_best_result {
                current_best_result = result;
            }
        }
    }

    Ok(current_best_result.to_string())
}
