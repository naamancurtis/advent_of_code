use std::sync::{mpsc, Arc};
use std::thread;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum Square {
    Clear,
    Tree,
}

impl From<char> for Square {
    fn from(c: char) -> Self {
        match c {
            '.' => Square::Clear,
            '#' => Square::Tree,
            _ => panic!(),
        }
    }
}

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> Vec<Vec<Square>> {
    input
        .lines()
        .map(|row| row.chars().map(Square::from).collect())
        .collect()
}

fn navigate(
    i: usize,
    j: usize,
    right_jump: usize,
    down_jump: usize,
    max_x_len: usize,
) -> (usize, usize) {
    let i = i + down_jump;
    let j = (j + right_jump) % max_x_len;
    (i, j)
}

pub fn taboggan_trajectory(input: &[Vec<Square>], right_jump: usize, down_jump: usize) -> u32 {
    let mut counter = 0;
    let mut i = 0;
    let mut j = 0;
    loop {
        let result = navigate(i, j, right_jump, down_jump, input[0].len());
        i = result.0;
        j = result.1;
        if i >= input.len() {
            break;
        }
        if input[i][j] == Square::Tree {
            counter += 1;
        }
    }
    counter
}

#[aoc(day3, part1)]
pub fn puzzle_1(input: &[Vec<Square>]) -> u32 {
    taboggan_trajectory(input, 3, 1)
}

#[aoc(day3, part2)]
pub fn run_scenarios(input: &[Vec<Square>]) -> u64 {
    let input = Arc::new(input.to_owned());

    // (right, down)
    let scenarios = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut handles = Vec::with_capacity(scenarios.len());
    let (tx, rx) = mpsc::channel();

    for (right, down) in scenarios.into_iter() {
        let tx_clone = tx.clone();
        let input = Arc::clone(&input);

        handles.push(thread::spawn(move || {
            let counter = taboggan_trajectory(&input, right, down);
            tx_clone.send(counter).unwrap();
        }));
    }

    drop(tx);

    for handle in handles {
        handle.join().unwrap();
    }

    rx.into_iter().map(|num| num as u64).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "..##.........##.........##.........##.........##.........##.......\n#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..\n.#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.\n..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#\n.#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.\n..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....\n.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#\n.#........#.#........#.#........#.#........#.#........#.#........#\n#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...\n#...##....##...##....##...##....##...##....##...##....##...##....#\n.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#\n";

    #[test]
    fn puzzle_1_test() {
        let input = parse_input(&TEST_INPUT);
        assert_eq!(taboggan_trajectory(&input, 3, 1), 7);
    }

    #[test]
    fn day_3_puzzle_2_test() {
        let input = parse_input(&TEST_INPUT);
        assert_eq!(run_scenarios(&input), 336);
    }
}
