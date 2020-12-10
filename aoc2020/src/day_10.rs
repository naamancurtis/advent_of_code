use std::collections::{HashMap, HashSet};

#[aoc_generator(day10)]
pub fn generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .filter_map(|l| l.parse::<u32>().ok())
        .collect()
}

#[aoc(day10, part1)]
pub fn puzzle_1(input: &[u32]) -> u32 {
    let (max, set) = input
        .iter()
        .fold((0, HashSet::new()), |(max, mut set), num| {
            set.insert(*num);
            (max.max(*num), set)
        });

    // Device always has a rating 3 higher than the last adapter
    let mut freq = [0, 0, 1];
    let mut joltage = 0;

    'outer: while joltage < max {
        for i in 0..3 {
            if set.get(&(joltage + i + 1)).is_some() {
                freq[i as usize] += 1;
                joltage += i + 1;
                continue 'outer;
            }
        }
    }

    freq[0] * freq[2]
}

#[aoc(day10, part2)]
pub fn puzzle_2(input: &[u32]) -> u64 {
    let (max, mut set) = input
        .iter()
        .fold((0, HashMap::new()), |(max, mut set), num| {
            set.insert(*num, None);
            (max.max(*num), set)
        });

    helper(0, &mut set, max)
}

fn helper(current_joltage: u32, set: &mut HashMap<u32, Option<u64>>, max: u32) -> u64 {
    if let Some(count) = set.get(&current_joltage).copied().flatten() {
        return count;
    }
    if current_joltage == max {
        return 1;
    }
    if current_joltage > max {
        return 0;
    }
    let mut permutations_to_end = 0;
    for i in 1..=3 {
        let new_joltage = current_joltage + i;
        let already_calculated = set.get(&new_joltage);
        if already_calculated.is_some() {
            if let Some(count) = already_calculated.copied().flatten() {
                permutations_to_end += count;
            } else {
                permutations_to_end += helper(new_joltage, set, max);
            }
        }
    }
    set.insert(current_joltage, Some(permutations_to_end));
    permutations_to_end
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn puzzle_1_test() {
        let input = generator(TEST_INPUT);
        assert_eq!(puzzle_1(&input), 220);
    }

    #[test]
    fn puzzle_2_test() {
        let input = generator(TEST_INPUT);
        assert_eq!(puzzle_2(&input), 19208);
    }
}
