use std::collections::{HashSet, VecDeque};

#[cfg(not(test))]
const PREAMBLE: usize = 25;

#[cfg(test)]
const PREAMBLE: usize = 5;

#[cfg(not(test))]
const WEAKNESS: i32 = 258585477;

#[cfg(test)]
const WEAKNESS: i32 = 127;

#[aoc_generator(day9)]
pub fn generator(input: &str) -> Vec<i32> {
    input
        .lines()
        .filter_map(|l| l.parse::<i32>().ok())
        .collect()
}

#[aoc(day9, part1)]
pub fn puzzle_1(input: &[i32]) -> i32 {
    let mut buffer = VecDeque::with_capacity(PREAMBLE);
    let mut set = HashSet::with_capacity(PREAMBLE);
    'outer: for (i, num) in input.iter().enumerate() {
        if i < PREAMBLE {
            buffer.push_front(num);
            set.insert(num);
            continue;
        }
        for candidate in &set {
            let target = num - *candidate;
            if target == **candidate {
                continue;
            }
            if set.contains(&target) {
                if let Some(prev_value) = buffer.pop_back() {
                    assert!(set.remove(&prev_value));
                    buffer.push_front(num);
                    set.insert(num);
                    continue 'outer;
                }
            }
        }
        return *num;
    }
    unreachable!("provided theres valid input")
}

#[aoc(day9, part2)]
pub fn puzzle_2(input: &[i32]) -> i32 {
    let mut left_ptr = 0;
    let mut right_ptr = 1;
    let mut current_total = input[0] + input[1];

    while right_ptr < input.len() {
        while current_total < WEAKNESS {
            right_ptr += 1;
            current_total += input[right_ptr];
        }
        while current_total > WEAKNESS {
            current_total -= input[left_ptr];
            left_ptr += 1;
        }
        if current_total == WEAKNESS {
            break;
        }
    }

    let mut max = std::i32::MIN;
    let mut min = std::i32::MAX;

    while left_ptr <= right_ptr {
        max = max.max(input[left_ptr]);
        min = min.min(input[left_ptr]);
        left_ptr += 1;
    }

    max + min
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn puzzle_1_test() {
        let input = generator(TEST_INPUT);
        assert_eq!(puzzle_1(&input), 127);
    }

    #[test]
    fn puzzle_2_test() {
        let input = generator(TEST_INPUT);
        assert_eq!(puzzle_2(&input), 62);
    }
}
