use std::collections::HashMap;

#[aoc_generator(day15)]
pub fn generator(input: &str) -> Vec<u32> {
    input
        .split(',')
        .filter_map(|n| n.parse::<u32>().ok())
        .collect()
}

#[aoc(day15, part1)]
pub fn puzzle_1(input: &[u32]) -> u32 {
    solution(input, 2020)
}

#[aoc(day15, part2)]
pub fn puzzle_2(input: &[u32]) -> u32 {
    solution(input, 30000000)
}

fn solution(input: &[u32], target: u32) -> u32 {
    let mut counter = input.len() as u32;
    let mut last_number = input[counter as usize - 1];
    let mut cache = input
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (i, n)| {
            acc.insert(*n, (0, (i + 1) as u32));
            acc
        });
    loop {
        counter += 1;
        if let Some((prev, curr)) = cache.get(&last_number) {
            if *curr != 0 && *prev != 0 {
                last_number = curr - prev;
            } else {
                last_number = 0;
            }
        } else {
            last_number = 0;
        }
        let entry = cache.entry(last_number).or_insert((0, 0));
        entry.0 = entry.1;
        entry.1 = counter;
        if counter == target {
            return last_number;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "0,3,6";

    #[test]
    fn puzzle_1_test() {
        let input = generator(TEST_INPUT);
        assert_eq!(puzzle_1(&input), 436);
    }

    #[test]
    fn puzzle_1_sol() {
        let input = generator("0,1,4,13,15,12,16");
        assert_eq!(puzzle_1(&input), 1665);
    }

    #[test]
    fn puzzle_2_test() {
        let input = generator(TEST_INPUT);
        assert_eq!(puzzle_2(&input), 175594);
    }
}
