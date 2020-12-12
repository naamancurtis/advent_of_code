use std::ops::Add;

#[aoc_generator(day12)]
pub fn generator(input: &str) -> Vec<Navigation> {
    input.lines().map(From::from).collect()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Navigation {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Forward(i32),
    Left(i32),
    Right(i32),
}

impl From<&str> for Navigation {
    fn from(s: &str) -> Self {
        let mut chars = s.chars();
        let direction = chars.next().expect("input should always be valid");
        let magnitude: i32 = chars
            .collect::<String>()
            .parse()
            .expect("number input should be valid");
        match direction {
            'N' => Navigation::North(magnitude),
            'S' => Navigation::South(magnitude),
            'E' => Navigation::East(magnitude),
            'W' => Navigation::West(magnitude),
            'F' => Navigation::Forward(magnitude),
            'L' => Navigation::Left(magnitude),
            'R' => Navigation::Right(magnitude),
            _ => panic!("invalid input"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Ship {
    pub facing: i32,
    pub east: i32,
    pub north: i32,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            facing: 90,
            east: 0,
            north: 0,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Waypoint {
    pub east: i32,
    pub north: i32,
}

impl Default for Waypoint {
    fn default() -> Self {
        Self { east: 10, north: 1 }
    }
}

impl Add<Navigation> for Ship {
    type Output = Self;

    fn add(mut self, nav: Navigation) -> Self::Output {
        match nav {
            Navigation::North(units) => {
                self.north += units;
                self
            }
            Navigation::South(units) => {
                self.north -= units;
                self
            }
            Navigation::East(units) => {
                self.east += units;
                self
            }
            Navigation::West(units) => {
                self.east -= units;
                self
            }
            Navigation::Forward(units) => match self.facing {
                0 => {
                    self.north += units;
                    self
                }
                90 => {
                    self.east += units;
                    self
                }
                180 => {
                    self.north -= units;
                    self
                }
                270 => {
                    self.east -= units;
                    self
                }
                _ => panic!("didn't expect to get non-90 degree based rotation"),
            },
            Navigation::Left(deg) => {
                self.facing -= deg;
                while self.facing.is_negative() {
                    self.facing += 360;
                }
                self
            }
            Navigation::Right(deg) => {
                self.facing += deg;
                while self.facing >= 360 {
                    self.facing -= 360;
                }
                self
            }
        }
    }
}

#[aoc(day12, part1)]
pub fn puzzle_1(input: &[Navigation]) -> i32 {
    let ship = input.iter().fold(Ship::default(), |ship, nav| ship + *nav);
    ship.east.abs() + ship.north.abs()
}

#[aoc(day12, part2)]
pub fn puzzle_2(input: &[Navigation]) -> i32 {
    let (ship, _) = input.iter().fold(
        (Ship::default(), Waypoint::default()),
        |(ship, waypoint), nav| navigate(ship, waypoint, *nav),
    );
    ship.east.abs() + ship.north.abs()
}

fn navigate(mut ship: Ship, mut waypoint: Waypoint, nav: Navigation) -> (Ship, Waypoint) {
    match nav {
        Navigation::North(units) => {
            waypoint.north += units;
        }
        Navigation::South(units) => {
            waypoint.north -= units;
        }
        Navigation::East(units) => {
            waypoint.east += units;
        }
        Navigation::West(units) => {
            waypoint.east -= units;
        }
        Navigation::Forward(units) => {
            ship.east += waypoint.east * units;
            ship.north += waypoint.north * units;
        }
        Navigation::Left(deg) => {
            let (x, y) = rotate(waypoint.east, waypoint.north, -deg);
            waypoint.east = x;
            waypoint.north = y;
        }
        Navigation::Right(deg) => {
            let (x, y) = rotate(waypoint.east, waypoint.north, deg);
            waypoint.east = x;
            waypoint.north = y;
        }
    }
    (ship, waypoint)
}

fn rotate(x: i32, y: i32, degrees: i32) -> (i32, i32) {
    let x = x as f64;
    let y = y as f64;
    let rad = (degrees as f64).to_radians();
    let rad_sin = rad.sin();
    let rad_cos = rad.cos();
    let xx = (x * rad_cos) + (y * rad_sin);
    let yy = (y * rad_cos) - (x * rad_sin);
    (xx.round() as i32, yy.round() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigate() {
        let waypoint = Waypoint { east: 10, north: 4 };
        let (_, result) = navigate(Ship::default(), waypoint, Navigation::Right(90));
        assert_eq!(
            result,
            Waypoint {
                east: 4,
                north: -10
            }
        )
    }

    const TEST_INPUT: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn puzzle_1_test() {
        let input = generator(TEST_INPUT);
        assert_eq!(puzzle_1(&input), 25);
    }

    #[test]
    fn puzzle_2_test() {
        let input = generator(TEST_INPUT);
        assert_eq!(puzzle_2(&input), 286);
    }
}
