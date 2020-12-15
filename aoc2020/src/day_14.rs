use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref REGEX: Regex =
        Regex::new(r"mem\[(?P<mem>\d+)\]\s=\s(?P<val>\d+)").expect("regex should be fine");
}

type PuzzleData = Vec<(String, Vec<(usize, usize)>)>;

#[aoc_generator(day14)]
pub fn generator(input: &str) -> PuzzleData {
    let mut result = Vec::new();
    let mut mask: String = String::default();
    let mut operations = Vec::new();
    for line in input.lines() {
        if line.contains("mask") {
            if !operations.is_empty() {
                result.push((mask.clone(), operations));
                operations = Vec::new();
            }
            if let Some(m) = line.split("= ").last() {
                mask = m.to_owned();
            }
            continue;
        }
        let captures = REGEX.captures(line).expect("should always find match");
        let addr: usize = captures
            .get(1)
            .expect("address should exist")
            .as_str()
            .parse()
            .expect("addr should be number");
        let value: usize = captures
            .get(2)
            .expect("value should exist")
            .as_str()
            .parse()
            .expect("value should be number");
        operations.push((addr, value));
    }
    result.push((mask, operations));
    result
}

#[aoc(day14, part1)]
pub fn puzzle_1(input: &[(String, Vec<(usize, usize)>)]) -> usize {
    let mut mem = HashMap::new();
    for (mask, ops) in input {
        for (addr, mut val) in ops {
            for (i, m) in mask.chars().rev().enumerate() {
                if m == 'X' {
                    continue;
                }
                let m = m.to_digit(10).expect("should be 0 or 1") as usize;
                val = change_bit_at_position(val, i, m);
            }
            mem.insert(*addr, val);
        }
    }
    mem.values().filter(|v| **v != 0).sum()
}

#[aoc(day14, part2)]
pub fn puzzle_2(input: &[(String, Vec<(usize, usize)>)]) -> usize {
    let mut mem = HashMap::new();
    for (mask, ops) in input {
        let mut has_calculated_permutations = false;
        let mut permutations = Vec::new(); // also acts as a cache for the mask;
        let mut floating_indexes = Vec::new();

        for (addr, val) in ops {
            let mut part_shifted_addr = *addr;

            for (i, m) in mask.chars().rev().enumerate() {
                if m == '1' {
                    part_shifted_addr = change_bit_at_position(part_shifted_addr, i, 1);
                    continue;
                }
                if m == 'X' {
                    // all floating bits start at `0`
                    part_shifted_addr = change_bit_at_position(part_shifted_addr, i, 0);
                    if !has_calculated_permutations {
                        floating_indexes.push(i);
                    }
                }
            }

            if !has_calculated_permutations {
                let index_len = floating_indexes.len();
                heaps_algorithm(&mut floating_indexes, index_len, &mut permutations);
                permutations = find_permutations(&permutations);
                has_calculated_permutations = true;
            }

            for perm in &permutations {
                let mut addr = part_shifted_addr;
                for p in perm {
                    addr = change_bit_at_position(addr, *p, 1);
                }
                mem.insert(addr, *val);
            }
        }
    }
    mem.values().filter(|v| **v != 0).sum()
}

fn change_bit_at_position(value: usize, position: usize, bit: usize) -> usize {
    let mask = 1 << position;
    (value & !mask) | ((bit << position) & mask)
}

// This is so hacky, but I don't know a better way of doing
fn find_permutations(input: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut set = HashSet::new();
    for i in 1..=input[0].len() {
        for perm in input {
            let mut temp_vec = perm[0..i].to_vec();
            temp_vec.sort_unstable();
            set.insert(temp_vec);
        }
    }
    set.insert(Vec::new());
    set.into_iter().collect()
}

fn heaps_algorithm(input: &mut Vec<usize>, n: usize, output: &mut Vec<Vec<usize>>) {
    if n == 1 {
        return;
    }

    for i in 0..n {
        heaps_algorithm(input, n - 1, output);

        if n % 2 == 1 {
            input.swap(0, n - 1);
            output.push(input.clone());
        } else {
            input.swap(i, n - 1);
            output.push(input.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_perms() {
        let mut input = vec![1, 2, 3];
        let mut perms = Vec::new();
        heaps_algorithm(&mut input, 3, &mut perms);
        let mut result = find_permutations(&perms);
        result.sort();
        assert_eq!(
            result.len(),
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![3],
                vec![1, 2],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3],
            ]
            .len()
        );
    }

    const TEST_INPUT: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    #[test]
    fn puzzle_1_test() {
        let input = generator(TEST_INPUT);
        assert_eq!(puzzle_1(&input), 165);
    }

    const TEST_INPUT_2: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    fn puzzle_2_test() {
        let input = generator(TEST_INPUT_2);
        assert_eq!(puzzle_2(&input), 208);
    }
}
