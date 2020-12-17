use std::cmp::{Eq, PartialEq};
use std::collections::HashSet;

const DIRS: [i32; 3] = [-1, 0, 1];

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Cube {
    w: i32,
    x: i32,
    y: i32,
    z: i32,
}

pub trait GenerateNeighbours {
    fn generate_neighbours(cube: &Cube, set: &HashSet<Cube>) -> (Vec<Cube>, i32);
}

pub struct ThirdDimension;
pub struct FourthDimension;

impl GenerateNeighbours for ThirdDimension {
    fn generate_neighbours(cube: &Cube, set: &HashSet<Cube>) -> (Vec<Cube>, i32) {
        let mut neighbours = Vec::with_capacity(80);
        let mut active_neighbours = 0;
        for x in &DIRS {
            for y in &DIRS {
                for z in &DIRS {
                    let c = Cube {
                        w: cube.w,
                        x: cube.x + x,
                        y: cube.y + y,
                        z: cube.z + z,
                    };
                    if c == *cube {
                        continue;
                    }
                    if set.get(&c).is_some() {
                        active_neighbours += 1;
                    }
                    neighbours.push(c);
                }
            }
        }
        (neighbours, active_neighbours)
    }
}

impl GenerateNeighbours for FourthDimension {
    fn generate_neighbours(cube: &Cube, set: &HashSet<Cube>) -> (Vec<Cube>, i32) {
        let mut neighbours = Vec::with_capacity(80);
        let mut active_neighbours = 0;
        for x in &DIRS {
            for y in &DIRS {
                for z in &DIRS {
                    for w in &DIRS {
                        let c = Cube {
                            w: cube.w + w,
                            x: cube.x + x,
                            y: cube.y + y,
                            z: cube.z + z,
                        };
                        if c == *cube {
                            continue;
                        }
                        if set.get(&c).is_some() {
                            active_neighbours += 1;
                        }
                        neighbours.push(c);
                    }
                }
            }
        }
        (neighbours, active_neighbours)
    }
}

#[aoc_generator(day17)]
pub fn generator(input: &str) -> HashSet<Cube> {
    let mut grid = HashSet::default();
    for (y, line) in input.lines().rev().enumerate() {
        for (i, c) in line.chars().enumerate() {
            let is_active = match c {
                '.' => false,
                '#' => true,
                _ => panic!("invalid input"),
            };
            if is_active {
                let cube = Cube {
                    x: i as i32,
                    y: y as i32,
                    z: 0,
                    w: 0,
                };
                grid.insert(cube);
            }
        }
    }
    grid
}

#[aoc(day17, part1)]
pub fn puzzle_1(input: &HashSet<Cube>) -> usize {
    solution::<ThirdDimension>(input)
}

#[aoc(day17, part2)]
pub fn puzzle_2(input: &HashSet<Cube>) -> usize {
    solution::<FourthDimension>(input)
}

fn solution<T: GenerateNeighbours>(input: &HashSet<Cube>) -> usize {
    let mut map = input.to_owned();
    for _ in 1..=6 {
        let mut temp_map = HashSet::new();
        std::mem::swap(&mut map, &mut temp_map);

        let mut cubes_to_add = HashSet::new();
        for cube in temp_map.iter() {
            let (neighbours, active_count) = T::generate_neighbours(cube, &temp_map);
            if active_count == 2 || active_count == 3 {
                map.insert(*cube);
            }
            for neighbour in neighbours {
                cubes_to_add.insert(neighbour);
            }
        }
        for cube in cubes_to_add.iter() {
            let (_, count) = T::generate_neighbours(cube, &temp_map);
            if count == 3 {
                map.insert(*cube);
            }
        }
    }
    map.into_iter().count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input_from_file;

    const TEST_INPUT: &str = ".#.
..#
###";

    #[test]
    fn puzzle_1_test() {
        let input = generator(TEST_INPUT);
        assert_eq!(puzzle_1(&input), 112);
    }

    #[test]
    fn puzzle_1_sol() {
        let input = get_input_from_file("input/2020/day17.txt");
        let input = generator(&input);
        assert_eq!(puzzle_1(&input), 359);
    }

    #[test]
    fn puzzle_2_test() {
        let input = generator(TEST_INPUT);
        assert_eq!(puzzle_2(&input), 848);
    }

    #[test]
    fn puzzle_2_sol() {
        let input = get_input_from_file("input/2020/day17.txt");
        let input = generator(&input);
        assert_eq!(puzzle_2(&input), 2228);
    }
}
