use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
pub struct Seat {
    pub row: u32,
    pub col: u32,
    pub id: u32,
}

impl PartialOrd for Seat {
    fn partial_cmp(&self, other: &Seat) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for Seat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

#[aoc_generator(day5)]
pub fn generator(input: &str) -> Vec<Seat> {
    input.lines().map(|l| parse_seat_information(l)).collect()
}

pub fn parse_seat_information(input: &str) -> Seat {
    let mut min = 0;
    let mut max = 127;
    let mut row = 0;
    let mut col = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            'F' | 'L' => max = (min + max) / 2,
            'B' | 'R' => min = ((min + max) / 2) + 1,
            _ => {
                // For now, just return the default seat
                return Seat::default();
            }
        }
        if i == 6 {
            row = if c == 'F' { min } else { max };
            min = 0;
            max = 7;
        } else if i == 9 {
            col = if c == 'L' { min } else { max };
        }
    }
    Seat {
        row,
        col,
        id: (row * 8) + col,
    }
}

#[aoc(day5, part1)]
pub fn find_highest_id(input: &[Seat]) -> u32 {
    if let Some(seat) = input.iter().max_by_key(|seat| seat.id) {
        return seat.id;
    }
    unreachable!();
}

#[aoc(day5, part2)]
pub fn find_missing_seat(input: &[Seat]) -> u32 {
    let mut input = input.to_vec();
    input.sort_unstable();
    let mut missing_id = 0;
    for seats in input.windows(2) {
        if seats.len() != 2 {
            continue;
        };
        if seats[1].id - 1 != seats[0].id {
            missing_id = seats[0].id + 1;
            break;
        }
    }
    missing_id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_5_check_parsing_logic() {
        let input = "FBFBBFFRLR";
        let expected = Seat {
            row: 44,
            col: 5,
            id: 357,
        };
        assert_eq!(parse_seat_information(input), expected)
    }
}
