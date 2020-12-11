// (i, j) ===> NOT (x, y)
const ADJACENT_MOVES: [(isize, isize); 8] = [
    (-1, -1), // Top left
    (-1, 0),  // Top
    (-1, 1),  // Top right
    (0, 1),   // Right
    (1, 1),   // Bottom Right
    (1, 0),   // Bottom
    (1, -1),  // Bottom Left
    (0, -1),  // Left
];

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Layout {
    EmptySeat,
    Occupied,
    Floor,
}

impl std::fmt::Debug for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Self::EmptySeat => 'L',
            Self::Occupied => '#',
            Self::Floor => '.',
        };
        write!(f, "{}", symbol)
    }
}

impl From<char> for Layout {
    fn from(c: char) -> Layout {
        match c {
            'L' => Layout::EmptySeat,
            '#' => Layout::Occupied,
            '.' => Layout::Floor,
            _ => panic!("invalid input"),
        }
    }
}

#[aoc_generator(day11)]
pub fn generator(input: &str) -> Vec<Vec<Layout>> {
    input
        .lines()
        .map(|l| l.chars().map(Layout::from).collect())
        .collect()
}

#[aoc(day11, part1)]
pub fn puzzle_1(input: &[Vec<Layout>]) -> i32 {
    let mut input = input.to_owned();
    let mut changes = Vec::new();
    loop {
        for i in 0..input.len() {
            for j in 0..input[i].len() {
                let adj_seats = check_number_of_occupied_adjacent_seats_1(&input, i, j);
                if adj_seats >= 4 && input[i][j] == Layout::Occupied {
                    changes.push((i, j, Layout::EmptySeat));
                }
                if adj_seats == 0 && input[i][j] == Layout::EmptySeat {
                    changes.push((i, j, Layout::Occupied));
                }
            }
        }
        if changes.is_empty() {
            break;
        }
        while let Some((i, j, state)) = changes.pop() {
            input[i][j] = state;
        }
    }
    input
        .iter()
        .map(|row| row.iter().filter(|seat| **seat == Layout::Occupied).count() as i32)
        .sum()
}

pub fn check_number_of_occupied_adjacent_seats_1(input: &[Vec<Layout>], i: usize, j: usize) -> u32 {
    let mut counter = 0;
    let i = i as isize;
    let j = j as isize;

    let col_len = input.len() as isize;
    let row_len = input[0].len() as isize;

    for (ii, jj) in &ADJACENT_MOVES {
        let new_i = i + ii;
        let new_j = j + jj;
        if new_i.is_negative() || new_j.is_negative() || new_i >= col_len || new_j >= row_len {
            continue;
        }
        if input[new_i as usize][new_j as usize] == Layout::Occupied {
            counter += 1;
        }
    }
    counter
}

#[aoc(day11, part2)]
pub fn puzzle_2(input: &[Vec<Layout>]) -> i32 {
    let mut input = input.to_owned();
    let mut changes = Vec::new();
    loop {
        for i in 0..input.len() {
            for j in 0..input[i].len() {
                let adj_seats = check_number_of_occupied_adjacent_seats_2(&input, i, j);
                if adj_seats >= 5 && input[i][j] == Layout::Occupied {
                    changes.push((i, j, Layout::EmptySeat));
                }
                if adj_seats == 0 && input[i][j] == Layout::EmptySeat {
                    changes.push((i, j, Layout::Occupied));
                }
            }
        }
        if changes.is_empty() {
            break;
        }
        while let Some((i, j, state)) = changes.pop() {
            input[i][j] = state;
        }
    }
    input
        .iter()
        .map(|row| row.iter().filter(|seat| **seat == Layout::Occupied).count() as i32)
        .sum()
}

pub fn check_number_of_occupied_adjacent_seats_2(input: &[Vec<Layout>], i: usize, j: usize) -> u32 {
    let mut counter = 0;
    let i = i as isize;
    let j = j as isize;

    let col_len = input.len() as isize;
    let row_len = input[0].len() as isize;

    'outer: for (ii, jj) in &ADJACENT_MOVES {
        let mut new_i = i;
        let mut new_j = j;

        loop {
            new_i += ii;
            new_j += jj;
            if new_i.is_negative() || new_j.is_negative() || new_i >= col_len || new_j >= row_len {
                continue 'outer;
            }
            if input[new_i as usize][new_j as usize] != Layout::Floor {
                break;
            }
        }

        if input[new_i as usize][new_j as usize] == Layout::Occupied {
            counter += 1;
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_checker_1() {
        let checking_input = "#.##.##.##
#######.##";
        let input = generator(checking_input);
        assert_eq!(check_number_of_occupied_adjacent_seats_1(&input, 0, 6), 3);
    }

    #[test]
    fn test_seat_checker_2() {
        let checking_input = ".......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....";
        let input = generator(checking_input);
        assert_eq!(check_number_of_occupied_adjacent_seats_2(&input, 4, 3), 8);
    }

    const TEST_INPUT: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn puzzle_1_test() {
        let input = generator(TEST_INPUT);
        assert_eq!(puzzle_1(&input), 37);
    }

    #[test]
    fn puzzle_2_test() {
        let input = generator(TEST_INPUT);
        assert_eq!(puzzle_2(&input), 26);
    }
}
