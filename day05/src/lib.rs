use color_eyre::eyre::{eyre, Error, Result};
use std::{collections::HashSet, ops::IndexMut};

// use anyhow::*;

#[derive(Debug)]
struct ParsedInput {
    stacks: Vec<Vec<char>>,
    moves: Vec<Move>,
}
#[derive(Debug)]
struct Move {
    amount: u8,
    from: u8,
    to: u8,
}

impl ParsedInput {
    fn parse_from_str(input: &str) -> Result<ParsedInput> {
        // let config = std::fs::read_to_string("cluster.json").context("asdasd")?;

        let mut lines = input.lines();

        let mut lines_crates = vec![];
        let number_of_stacks = loop {
            let line = lines.next().ok_or(eyre!("wrong input"))?;
            if line.trim_start().starts_with("1") {
                break line
                    .split_whitespace()
                    .last()
                    .ok_or(eyre!("wrong input"))?
                    .parse::<_>()?;
            }
            lines_crates.push(line);
        };

        let mut stacks: Vec<Vec<char>> = Vec::with_capacity(number_of_stacks);
        (0..number_of_stacks).for_each(|_| stacks.push(vec![]));
        for line in lines_crates.iter().rev() {
            // i wanted to use "line.chars().array_chunks();" but it's ustable"
            // so I will convert it to byte array (input has only asci character anyway)
            line.as_bytes()
                .chunks(4)
                .enumerate()
                .for_each(|(idx, slice)| {
                    let crate_id = slice[1];
                    if crate_id != b' ' {
                        stacks[idx].push(crate_id as char);
                    }
                })
        }

        lines.next(); // empty line

        let moves = lines
            .map(|line| {
                // dbg!(line);
                let mut split_iter = line.split_whitespace();
                split_iter.next(); // "move"
                let amount = split_iter.next().unwrap().parse().unwrap();
                split_iter.next(); // "from"
                let from = split_iter.next().unwrap().parse().unwrap();
                split_iter.next(); // "to"
                let to = split_iter.next().unwrap().parse().unwrap();

                Move { amount, from, to }
            })
            .collect::<Vec<_>>();

        Ok(ParsedInput { stacks, moves })
    }
}

pub fn run_part1_v1(input: &str) -> Result<String> {
    let parsed_input = ParsedInput::parse_from_str(input)?;
    let ParsedInput { mut stacks, moves } = parsed_input;

    for m in moves {
        let from_stack = &mut stacks[m.from as usize - 1];
        let mut tmp_vec = vec![];
        for _ in 0..m.amount {
            tmp_vec.push(from_stack.pop().unwrap());
        }

        let to_stack = &mut stacks[m.to as usize - 1];
        for c in tmp_vec {
            to_stack.push(c);
        }

        // This was interesting. I had to introduce this "tmp_vec" to satisfy rust borrow checker.
        // I couldn't get references to two different mutable vectors inside a vector
        // Next versions of this function found better ways to do it.
    }

    let mut result = String::with_capacity(stacks.len());
    for s in stacks {
        let top_crate_for_stack = s.last().unwrap();
        result.push(*top_crate_for_stack);
    }

    Ok(result)
}

pub fn run_part1_v2(input: &str) -> Result<String> {
    let parsed_input = ParsedInput::parse_from_str(input)?;
    let ParsedInput { mut stacks, moves } = parsed_input;

    for m in moves {
        for _ in 0..m.amount {
            // here I moved referencing the vector inside the loop so Rust with NLL (non-lexical lifetimes)
            // will be able to figure out that this code is safe (I'm not mutably borrowing at the same time)
            let from_stack = &mut stacks[m.from as usize - 1];
            let crate_to_move = from_stack.pop().unwrap();
            let to_stack = &mut stacks[m.to as usize - 1];
            to_stack.push(crate_to_move)
        }
    }

    let mut result = String::with_capacity(stacks.len());
    for s in stacks {
        let top_crate_for_stack = s.last().unwrap();
        result.push(*top_crate_for_stack);
    }

    Ok(result)
}

pub fn run_part1_v3(input: &str) -> Result<String> {
    let parsed_input = ParsedInput::parse_from_str(input)?;
    let ParsedInput { mut stacks, moves } = parsed_input;

    for m in moves {
        //using std::mem::take to get ownership of vector inside vector.
        let mut from_stack = std::mem::take(&mut stacks[m.from as usize - 1]);
        let mut to_stack = std::mem::take(&mut stacks[m.to as usize - 1]);
        to_stack.reserve(m.amount as _);
        for _ in 0..m.amount {
            let crate_to_move = from_stack.pop().unwrap();
            to_stack.push(crate_to_move);
        }
        //we need to put it back to where we've take it from
        stacks[m.from as usize - 1] = from_stack;
        stacks[m.to as usize - 1] = to_stack;
    }

    let mut result = String::with_capacity(stacks.len());
    for s in stacks {
        let top_crate_for_stack = s.last().unwrap();
        result.push(*top_crate_for_stack);
    }

    Ok(result)
}

pub fn run_part2_v1(input: &str) -> Result<String> {
    let parsed_input = ParsedInput::parse_from_str(input)?;
    let ParsedInput { mut stacks, moves } = parsed_input;

    for m in moves {
        let from_stack = &mut stacks[m.from as usize - 1];
        let mut tmp_vec = vec![];
        for _ in 0..m.amount {
            tmp_vec.push(from_stack.pop().unwrap());
        }

        let to_stack = &mut stacks[m.to as usize - 1];
        for c in tmp_vec.iter().rev() {
            to_stack.push(*c);
        }
    }

    let mut result = String::with_capacity(stacks.len());
    for s in stacks {
        let top_crate_for_stack = s.last().unwrap();
        result.push(*top_crate_for_stack);
    }

    Ok(result)
}

pub fn run_part2_v2(input: &str) -> Result<String> {
    let parsed_input = ParsedInput::parse_from_str(input)?;
    let ParsedInput { mut stacks, moves } = parsed_input;

    for m in moves {
        let mut from_stack = std::mem::take(&mut stacks[m.from as usize - 1]);

        //Here we are using draing method to remove range of elementf from one vector
        //This is fast because there is no actual removal or reallocation happening inside vector.
        //The returned iterator is actually a slice to the internal slice of origina vector.
        let removed = from_stack.drain((from_stack.len() - m.amount as usize)..);

        let to_stack = &mut stacks[m.to as usize - 1];
        to_stack.reserve(m.amount as _);
        for c in removed {
            to_stack.push(c);
        }

        stacks[m.from as usize - 1] = from_stack;
    }

    let mut result = String::with_capacity(stacks.len());
    for s in stacks {
        let top_crate_for_stack = s.last().unwrap();
        result.push(*top_crate_for_stack);
    }

    Ok(result)
}

// Same as "v2", but using Vec.extend to add "removed" Drain iterator
pub fn run_part2_v3(input: &str) -> Result<String> {
    let parsed_input = ParsedInput::parse_from_str(input)?;
    let ParsedInput { mut stacks, moves } = parsed_input;

    for m in moves {
        let mut from_stack = std::mem::take(&mut stacks[m.from as usize - 1]);

        let removed = from_stack.drain((from_stack.len() - m.amount as usize)..);

        let to_stack = &mut stacks[m.to as usize - 1];

        to_stack.extend(removed);

        stacks[m.from as usize - 1] = from_stack;
    }

    let mut result = String::with_capacity(stacks.len());
    for s in stacks {
        let top_crate_for_stack = s.last().unwrap();
        result.push(*top_crate_for_stack);
    }

    Ok(result)
}

// Same as "v3", but with "better extend" (extend_from_slice)
// The speed is similar. I checked the output asm and this function
// has less instructions thatn "v3"
// TODO: do some testing and ask on reddit between difference of:
// let removed = vec1.drain((vec1.len() - 5)..); vec.extend(removed);
// let removed = vec1.drain((vec1.len() - 5)..); vec.extend_from_slice(removed.as_slice());
pub fn run_part2_v4(input: &str) -> Result<String> {
    let parsed_input = ParsedInput::parse_from_str(input)?;
    let ParsedInput { mut stacks, moves } = parsed_input;

    for m in moves {
        let mut from_stack = std::mem::take(&mut stacks[m.from as usize - 1]);

        let removed = from_stack.drain((from_stack.len() - m.amount as usize)..);

        let to_stack = &mut stacks[m.to as usize - 1];

        to_stack.extend_from_slice(removed.as_slice());
        drop(removed);

        stacks[m.from as usize - 1] = from_stack;
    }

    let mut result = String::with_capacity(stacks.len());
    for s in stacks {
        let top_crate_for_stack = s.last().unwrap();
        result.push(*top_crate_for_stack);
    }

    Ok(result)
}

//Version that use my "index_two_mut" method to get two mutable references to vector element
pub fn run_part2_v5(input: &str) -> Result<String> {
    let parsed_input = ParsedInput::parse_from_str(input)?;
    let ParsedInput { mut stacks, moves } = parsed_input;

    for m in moves {
        let (from_stack, to_stack) = stacks.index_two_mut(m.from as usize - 1, m.to as usize - 1);
        let removed = from_stack.drain((from_stack.len() - m.amount as usize)..);
        to_stack.extend_from_slice(removed.as_slice());
    }

    let mut result = String::with_capacity(stacks.len());
    for s in stacks {
        let top_crate_for_stack = s.last().unwrap();
        result.push(*top_crate_for_stack);
    }

    Ok(result)
}

//version that uses my "index_many_mut" methods to get mutable references to vector element.
pub fn run_part2_v6(input: &str) -> Result<String> {
    let parsed_input = ParsedInput::parse_from_str(input)?;
    let ParsedInput { mut stacks, moves } = parsed_input;

    for m in moves {
        let [from_stack, to_stack] =
            stacks.index_many_mut([m.from as usize - 1, m.to as usize - 1]);
        let removed = from_stack.drain((from_stack.len() - m.amount as usize)..);
        to_stack.extend_from_slice(removed.as_slice());
    }

    let mut result = String::with_capacity(stacks.len());
    for s in stacks {
        let top_crate_for_stack = s.last().unwrap();
        result.push(*top_crate_for_stack);
    }

    Ok(result)
}

//Experimentations with methods that return more that one mutable references to elements in indexable container.
//Actually later I found there is something simillar planned for inclusion in std:
//https://github.com/rust-lang/rust/issues/104642

//A method for a vector to return two mutable references to it's element.
//It should be possible to implement in unsafe code and provide safe wrapper for it. We just need
//to test if provided indexes don't overlap.
trait IndexMutTwoCustom<T>
where
    Self: IndexMut<usize, Output = T>,
{
    fn index_two_mut(&mut self, idx0: usize, idx1: usize) -> (&mut T, &mut T) {
        {
            if idx0 == idx1 {
                panic!("Get two mutable reference to the same object is forbidden")
            }
            unsafe {
                (
                    &mut *(&mut self[idx0] as *mut T),
                    &mut *(&mut self[idx1] as *mut T),
                )
            }
        }
    }
}
impl<T> IndexMutTwoCustom<T> for Vec<T> {}

trait IndexMutMany<T, const N: usize>
where
    Self: IndexMut<usize, Output = T>,
{
    //Generic version for any size of array.
    //It is mych slower then specialized version implemented for Vec<T>
    //because it allocated HashSet.
    fn index_many_mut(&mut self, idxs: [usize; N]) -> [&mut T; N] {
        let mut hs = HashSet::with_capacity(N);
        for i in idxs {
            if !hs.insert(i) {
                panic!("Provided not unique set of indexes");
            }
        }

        unsafe {
            idxs.map(|i| {
                let ptr: *mut T = &mut self[i];
                &mut *ptr
            })
        }
    }
}

impl<T> IndexMutMany<T, 2> for Vec<T> {
    fn index_many_mut(&mut self, idxs: [usize; 2]) -> [&mut T; 2] {
        let [idx0, idx1] = idxs;
        if idx0 == idx1 {
            panic!("Provided not unique set of indexes");
        }

        unsafe {
            [
                &mut *(&mut self[idx0] as *mut T),
                &mut *(&mut self[idx1] as *mut T),
            ]
        }
    }
}

impl<T> IndexMutMany<T, 3> for Vec<T> {
    fn index_many_mut(&mut self, idxs: [usize; 3]) -> [&mut T; 3] {
        let [idx0, idx1, idx2] = idxs;
        if idx0 == idx1 || idx0 == idx2 || idx1 == idx2 {
            panic!("Provided not unique set of indexes");
        }

        unsafe {
            [
                &mut *(&mut self[idx0] as *mut T),
                &mut *(&mut self[idx1] as *mut T),
                &mut *(&mut self[idx2] as *mut T),
            ]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    static INPUT_EXAMPLE: &str = include_str!("../input_example");

    use std::sync::Once;

    static INIT: Once = Once::new();

    pub fn initialize() {
        INIT.call_once(|| {
            color_eyre::install().unwrap();
            println!("elllo initialize calling")
        });
    }

    #[test]
    fn test_part1_v1() -> Result<()> {
        initialize();
        let result_input_example = run_part1_v1(INPUT_EXAMPLE)?;
        assert_eq!(result_input_example, "CMZ");
        Ok(())
    }

    #[test]
    fn test_part1_v2() -> Result<()> {
        initialize();
        let result_input_example = run_part1_v2(INPUT_EXAMPLE)?;
        assert_eq!(result_input_example, "CMZ");
        Ok(())
    }

    #[test]
    fn test_part2_v1() -> Result<()> {
        initialize();
        let result_input_example = run_part2_v1(INPUT_EXAMPLE)?;
        assert_eq!(result_input_example, "MCD");
        Ok(())
    }

    #[test]
    fn test_part2_v2() -> Result<()> {
        initialize();
        let result_input_example = run_part2_v2(INPUT_EXAMPLE)?;
        assert_eq!(result_input_example, "MCD");
        Ok(())
    }

    #[test]
    fn test_part2_v3() -> Result<()> {
        initialize();
        let result_input_example = run_part2_v3(INPUT_EXAMPLE)?;
        assert_eq!(result_input_example, "MCD");
        Ok(())
    }

    #[test]
    fn test_part2_v4() -> Result<()> {
        initialize();
        let result_input_example = run_part2_v4(INPUT_EXAMPLE)?;
        assert_eq!(result_input_example, "MCD");
        Ok(())
    }

    #[test]
    fn test_part2_v5() -> Result<()> {
        initialize();
        let result_input_example = run_part2_v5(INPUT_EXAMPLE)?;
        assert_eq!(result_input_example, "MCD");
        Ok(())
    }

    #[test]
    fn test_part2_v6() -> Result<()> {
        let result_input_example = run_part2_v6(INPUT_EXAMPLE)?;
        assert_eq!(result_input_example, "MCD");
        Ok(())
    }
}
