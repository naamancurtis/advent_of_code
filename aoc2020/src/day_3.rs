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

pub fn parse_input(input: &[&str]) -> Vec<Vec<Square>> {
    input
        .iter()
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

mod puzzle_2 {
    use super::*;
    use std::sync::{mpsc, Arc};
    use std::thread;

    pub fn run_scenarios(input: &[&str]) -> u64 {
        let input = Arc::new(parse_input(input));

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
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: [&str; 11] = [
        "..##.........##.........##.........##.........##.........##.......",
        "#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..",
        ".#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.",
        "..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#",
        ".#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.",
        "..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....",
        ".#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#",
        ".#........#.#........#.#........#.#........#.#........#.#........#",
        "#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...",
        "#...##....##...##....##...##....##...##....##...##....##...##....#",
        ".#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#",
    ];

    #[test]
    fn puzzle_1_test() {
        let input = parse_input(&TEST_INPUT);
        assert_eq!(taboggan_trajectory(&input, 3, 1), 7);
    }

    #[test]
    fn puzzle_1_sol() {
        let input = parse_input(&INPUT);
        assert_eq!(taboggan_trajectory(&input, 3, 1), 244);
    }

    #[test]
    fn day_3_puzzle_2_test() {
        assert_eq!(puzzle_2::run_scenarios(&TEST_INPUT), 336);
    }

    #[test]
    fn day_3_puzzle_2_sol() {
        assert_eq!(puzzle_2::run_scenarios(&INPUT), 9406609920);
    }
}

const INPUT: [&str; 323] = [
    "....#...#####..##.#..##..#....#",
    "..##.#.#.........#.#......##...",
    "#.#.#.##.##...#.......#...#..#.",
    "..##.............#.#.##.....#..",
    "##......#.............#....#...",
    ".....##..#.....##.#.......##..#",
    ".##.....#........##...##.#....#",
    ".##......#.#......#.....#..##.#",
    "##....#..#...#...#...##.#...##.",
    "##........##.#...##......#.#.#.",
    "..#.#........#...##.....#.....#",
    "..#.......####.#....#..#####...",
    ".##..#..#..##.#.....###.#..#...",
    "......###..##.....#.#.#..###.#.",
    "..#.#...#..##.....#....#.#.....",
    ".....#.#...#.###.#..#..........",
    "##.....#...#.#....#..#.#.......",
    "..#...#...#.........##......#..",
    "......#.#...#...#..#...##.#...#",
    "....#.................##.##....",
    "...#......#.............#....##",
    "##..#..#..........#...##.#.#...",
    "....#...##....#..#.#...........",
    "##.#.#.#...#....#........#..#.#",
    "...###..........#...#...#..##.#",
    "..##.......###.#......##.##....",
    "...........#.#....#.....#.#...#",
    "..#......##.#...##.#.#......#.#",
    "..........#.#....#.#..#....#...",
    "##..##...##.......#.#....#.#.##",
    ".##..#.#..#...........#.#...#.#",
    "#......##......#....####.#....#",
    "..###......##...#...#.#.......#",
    ".#.##.##....##..#..##...#......",
    ".#....#..#........#..#.##.#.#..",
    "..#.........#.#.###....###.#...",
    "..#..#.#.#..#..#.##.##...####..",
    "#..#..#......#..#.#....#.#.#.##",
    "..#.........#...#..#.#.#..#...#",
    "#..#......###.....##....##.....",
    "#..#.....#.#.#.##.....##...#.#.",
    "##..#.#...#.........#.#........",
    "#....#.......#.....#..#..#.#...",
    "...###.##.###.###.#####..#...#.",
    ".....#..#.#..##...............#",
    "..#.....###.###.#.....#.#....##",
    "###.#.........#..#.#.#..#.#..#.",
    ".##.........#..#..##....#.#...#",
    ".#...#........#...#.....#....#.",
    "####..........###....#.#.#....#",
    "....##..###....#....#.#...#....",
    "..............##......##..#.###",
    ".#...........###.#.#....#......",
    "###.#..#..#...#.........##.....",
    "..#.....##...#.#.....##...#.##.",
    ".###.#........#..#.#...#.#..##.",
    ".......##....##.........##.#..#",
    "#....#...#...##...#.#..#..#..##",
    "...#...##..#...#...#.#....#.#.#",
    "#.#......#.#...##......#.#...##",
    ".#.###..#.###.#.....#.##.##.#.#",
    "#...#............#...#.##..##..",
    "....#..###.......#.....##....#.",
    ".#####..#.....#.....#...#.....#",
    "..##..#..###.......##.#........",
    ".#...##.##.....#.##...##...#..#",
    "......###...#....#....#........",
    "....#...#.#....#...#.#.#......#",
    "....#..##...##.#..#....###.....",
    "...#...#..#.#...#....#.#..#####",
    "####....#.....#.........#.#....",
    "...###.#.#..#.#..##............",
    ".##..#####..#...#..#..#.......#",
    ".###......#.#.#..#....#.....#..",
    "#....##.##..#.#...............#",
    "...#.#..#........#......#....#.",
    "#.....#....###....#..#.#.#.....",
    ".#..#....#...#.#.....##....#...",
    "..#.##.#.##.#..#.##.#.....#.#..",
    ".......#.......###..###..#...#.",
    ".#.......#..#........#.#.......",
    ".#.#...#.....#.##..##.....#....",
    "#.......##....#......#.....##..",
    ".#.....#...##...#..##.....#....",
    "....#..#.#.......#.#.#.........",
    "..#....#....##.##..#..##.##.#..",
    ".#...#....##...#........#....#.",
    "#.#......#...#....#...........#",
    ".#....#..#..###.#.....#..#.....",
    "..#..................#.....#...",
    "..#...###..#..####.#..#.#.#.#..",
    "...#........##...##..##..#....#",
    "...#.....#........##..#.....#.#",
    "#....#.....##.##......#...##...",
    "...#####....#..##..##...#.#....",
    "###.........#.#..#..#..##.#...#",
    "##...#..##...#.##.#........#...",
    ".#....#.#...#..#...#..#.#......",
    ".#......##.#...#...#..#....#...",
    "#..#.#.#......##.##.####..#....",
    ".#...#.#.##...##.#...#...#.....",
    "####.#.........#...##..##....##",
    ".....###........###.##...#.#...",
    ".##.....#.....#....##.....##...",
    ".#.#...#####....##...##.....#..",
    "....###..........#......##..#.#",
    "..#.....#....#..#...#.....##..#",
    "...##.##.#.######....#.#....##.",
    "...#.#.#...#..#....##.........#",
    ".#.#...##...#....#.#....##.....",
    "...#..#.....#.....#.##.....#...",
    ".#.#.#.....#.##.#....#.#....##.",
    "#...#......###...#..###...#....",
    "...##.#.#..#........##.......#.",
    ".####.####......#........#.....",
    "....#..#####....#......####....",
    "#...##.#..#..#####.#...#......#",
    "#.#....#..#.........###........",
    ".##.........#....#......#.#....",
    "...###.........####.#........##",
    "..#..#........#.#..##......#..#",
    ".##..#....#...##.####.#...#....",
    "......#.......#..#..#.#.##.#...",
    ".###....#.#...#.#.......##..#.#",
    "#...#....#............#####....",
    "...#.##......#####..#........#.",
    "..#...##.....#...#..#.#........",
    "...#.#...#...##...#..#....#....",
    "..#..##.....#....#.#.###.......",
    ".......##..#...#.............##",
    ".....#.....#..##.##.....#......",
    ".....##...#......#..##....#.###",
    ".#...#.#.#.#.##.....##..###..#.",
    "....##..........#.....###......",
    "....#...#.#.#..#.......#....#..",
    "..###...#...........##..###....",
    "...#.##.......#....#....#.#....",
    "##...#..##..#.##..........##..#",
    ".##.....#..#......##..####.#.##",
    "....##..#.#.###......#..#...#..",
    "####..#.#....#...#....###.#.#..",
    "###......#...##.##..#.##..#..#.",
    "..#.#..#.#.#.....#...#..#.####.",
    ".###.#...##...##....##......#.#",
    "..#............#.##..#....#..#.",
    "###.......#......###..#........",
    "....##......###.....#.#..###...",
    "..#...##...#......#..#.........",
    "#..####.#....#.....###....#.#..",
    ".#.#.#.......##....###.........",
    ".......#.##.#####....#.#...####",
    ".#...#....#....#.###..#.....#..",
    ".###.#.#.###.###.#..####.##.#..",
    "....#.........#.#.......##.....",
    "#..#..#.#...........#.#.##..#.#",
    ".#.....#..#...#.....#.##......#",
    "..###.#............#.....####..",
    "#.....##..##...#....####....#..",
    "...#.....#..................#..",
    "....#.###.#..#..#..##..#..##...",
    "...##.#........#......#...##...",
    "#................##....#...#...",
    "..##......##.#.##..#....#.....#",
    ".#..#.....#..........##.#.#....",
    ".....#...####....#..#......#...",
    "..#......###.#.#.#.#.......#..#",
    ".##......#.......#....###.#....",
    "#..#.#.#..#...#.#.##..##..#....",
    "....#...##..#.#......#.##...#..",
    "...###...#.##..#...#....#......",
    "##......#.#...#.#.#.........#..",
    "..#..........#...###.#.##....#.",
    "...##.....#.....#...###..#.....",
    "..####.#.....#.#.....#..#.#....",
    ".#.....##...##.##.#.....#.####.",
    ".......#.....#...##..........#.",
    ".#...#.#....#####....###.#..#.#",
    ".##.##....##...##.#.....#......",
    "#......#.##..#..##.#.#.......#.",
    ".#..#....###..#........##...#..",
    "..#......##.......###..##...#..",
    ".#..........#.#.......##.....#.",
    "....##.....##.#.#.##........##.",
    "..#.#..###..#..##...#.##...#...",
    ".......#.....#..#...#...#.....#",
    "##.#...#.#.#.##........#......#",
    "..###.....##..#....#.......##..",
    "#####..####...#.#..##.#...#..#.",
    "#...####....#........#....#....",
    ".#.#.#..#...##....#.......#.#..",
    "...#....##..##..#..#..#####.###",
    "...#......#.#..#......#...####.",
    ".##.....##.##.#.####.#..##...#.",
    "....#..#..##..##....#....#...##",
    "##.###........#...##....#.....#",
    "..#.#.#.......#....#..#....#...",
    "......##.....##....#...#.....#.",
    "#.#..#.#.......#....#.#.#......",
    ".....###..#...#.....#..##..#...",
    "......###.....#.#.#...#...##..#",
    ".#..#.#....##...#...#........#.",
    "#..###.#...####.#...#..........",
    ".#.##.#..#..##..#..###..##...#.",
    ".......#.#..........#.........#",
    "#......###..##..#....###.......",
    "..#............#.#........#...#",
    "..##.#.............#......#..##",
    ".#....#..#.#..#....###..#...#..",
    "....##....#..##...###....#....#",
    ".#....###.............#........",
    "#..#...#..#....#.##.#.....##...",
    "...........#.....#....#....#...",
    ".##.##.#...#....##......##..##.",
    "......#.#.##.#..##........#...#",
    "....##...##...#...#...#.#......",
    ".#...#....#...#......#.#...#..#",
    "........##.....#.#..#...##..##.",
    "##...#.....#.....####...#..#...",
    ".#.#..##.##......#...#.#...#...",
    "##...##.#......#....#.######.#.",
    "##.....####.###......#.##.#....",
    ".#.##....##........#...#..####.",
    ".......#..#....##...#.#...#..#.",
    "...##..........#..#........#..#",
    ".##.....#...#...#.##.###.......",
    ".##....#...#.#..#.....#...#....",
    "..#...#.....#.####.#.........#.",
    "#...#.##...#.#..#.#..#.###.#..#",
    ".##..#.#.##.........####....###",
    ".#..##........#..#.......#.....",
    "......#.#####.#.........#.#...#",
    "......#....#.#####...........#.",
    "..##....##..#.#..#....#......##",
    "#.#......#.##.#.##....#....#.#.",
    "..#..##.#...#.......##.........",
    ".....##.#...#..........#.......",
    "...#........#..#...#.....##.###",
    "....##.........#...#.#.....#...",
    ".......#.#....#...#.......#...#",
    ".#..#...##....#..#...........#.",
    ".#....##.##.#..#..####.#.#.....",
    ".##........#.....#..#......##..",
    ".#..##......#......#..##..#....",
    "###.....##.......#..##.#.......",
    ".....##......#.#...##...##.....",
    ".##....##..#..#####...#...#.##.",
    "##...##.#....##.#.#.#....##....",
    ".#.....#...#......#......##....",
    "##.#............#...#....#.....",
    "#..#.....#.....##.##.##..#..##.",
    "......#..............#..#...#.#",
    "....#.#....##......#..#...#....",
    ".#...#..#...#......#..##....#.#",
    ".....#......#..##...#.#....#...",
    "#...............#.##..#......#.",
    ".....#..##.#..#.#...###.....#..",
    "...#..#..#...#....#..#..##.#...",
    ".#...#...#..#......##...#......",
    "....###............#.#.#....#..",
    "#.#...#..#..#.#....#........#..",
    "....#.#.#..#..#....#..#...##.#.",
    ".#....#.#...#....#......#.#...#",
    "##..#.#.#..#.....#...###....#.#",
    ".##.....#.#...#..........#..#.#",
    "#....#......#....#.#.#...#.....",
    "#.#.....#.###.......#..#..#.#..",
    "#.....##..#.###...#...##...#.##",
    ".#.##....#.#.....##......###...",
    ".#.......##................#...",
    ".........#........####......#..",
    "...##.###..#.....#.#.....##.#..",
    "..#....#.#.#.##..........#.....",
    "#..##.....#.............##.....",
    ".##...#..#.......#.......#..#..",
    "...#.................#......##.",
    "....#....#.....#..###.#....###.",
    "..#.#..#...#..#.....###....#.#.",
    ".....###...#....#....#.#.##..##",
    "...#...#.........####........#.",
    ".......#..##.........#.........",
    ".#......##.....#.#####...##....",
    "....#.###...#.#....##..#......#",
    ".##..#....#.#....#..#.###.....#",
    "..#...#.#...#.##.....#.#....##.",
    "#.#.#.#.....##...#.#..##..#....",
    ".#............#.#.#..#...#...#.",
    "...##.#..#..####.#.###..##.....",
    ".##.....#.......#..##.#...##.#.",
    "#.#...#..#.##...##...####..#..#",
    "...##.......#.#.#.#.#.#...#..##",
    "...#.#.##..##..............###.",
    ".....#...#........#...#......#.",
    "..#..#..##....#..##.#.....#....",
    "#.....##........#.........##.#.",
    "###..#....#.##..##.............",
    ".#..#...#.#......#..#..##.....#",
    "...#.#.#............##........#",
    "..#.#....#..#....##....#...##.#",
    "...##...#...#..........##.#####",
    "....#..#.#.......##....#.#.###.",
    "##..#..#..#...###.#.....#......",
    "....###.#.#.#.##..##.#...#.....",
    ".....####..#.#..#.#......#.#.#.",
    "#.....#...#..#.#.........#..#..",
    ".##....#.#.####......##..#..##.",
    "......#.##.#.#..#..#....#.#....",
    ".#..#...#...#...#..#.....#.....",
    "..##.#..............#......#...",
    ".....###.##.......#.....#..#...",
    "..#.#..#..#.......#...##.##..#.",
    "##.###......#......#.#..#..##..",
    "..##.....#..#..#......#..#.....",
    "...##.......#.#..#.........#.#.",
    "......##.##.#.......#..#.#.....",
    "#......#........##..#.......#.#",
    "###....#...#...#.#...#..#..#...",
    "#..###....#....####..#...#.....",
    "....##..#.##.#....#..##...#.#..",
    "#.##..#....##..#...#..#.#.#..#.",
    "#.........#.....#...#.......#..",
    "...#.....#.#.....#........##...",
    "..#.##..#......#...#.....##.#..",
    "...###....#.....#...#..#.##..#.",
];
