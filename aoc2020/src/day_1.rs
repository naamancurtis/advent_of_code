use std::{cmp::Ordering, collections::HashSet};

const TARGET_SUM: u32 = 2020;

#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .filter_map(|l| l.parse::<u32>().ok())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_expense_report(input: &[u32]) -> u32 {
    let mut set = HashSet::new();
    for num in input {
        if let Some(existing_num) = set.get(&(TARGET_SUM - num)) {
            return *existing_num * num;
        }
        set.insert(num);
    }
    unreachable!()
}

#[aoc(day1, part2)]
pub fn solve_expense_report_2(input: &[u32]) -> u32 {
    let mut input = input.to_vec();
    input.sort_unstable(); // nlog(n)

    for i in 0..input.len() - 1 {
        let current_target = TARGET_SUM - input[i];
        if let Some((a, b)) = helper(&input[i + 1..], current_target) {
            return a * b * input[i];
        }
    }
    0
}

fn helper(input: &[u32], target: u32) -> Option<(u32, u32)> {
    let mut left_ptr = 0;
    let mut right_ptr = input.len() - 1;
    while left_ptr <= right_ptr {
        match (input[left_ptr] + input[right_ptr]).cmp(&target) {
            Ordering::Equal => return Some((input[left_ptr], input[right_ptr])),
            Ordering::Less => {
                left_ptr += 1;
            }
            Ordering::Greater => {
                right_ptr -= 1;
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: [u32; 6] = [1721, 979, 366, 299, 675, 1456];

    #[test]
    fn puzzle_1() {
        assert_eq!(solve_expense_report(&TEST_INPUT), 514579);
    }

    #[test]
    fn puzzle_2() {
        assert_eq!(solve_expense_report_2(&TEST_INPUT), 241861950);
    }
}
