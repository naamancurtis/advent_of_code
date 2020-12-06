use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Seat {
    pub row: u32,
    pub col: u32,
    pub id: u32,
}

fn get_input_from_file() -> Vec<String> {
    let file = File::open("./data/day_5.txt").expect("file failed to open");
    let reader = BufReader::new(file);
    reader.lines().filter_map(|s| s.ok()).collect()
}

pub fn parse_seat_information(input: String) -> Seat {
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

mod puzzle_1 {
    use super::*;

    pub fn find_highest_id() -> u32 {
        let input = get_input_from_file();
        if let Some(seat) = input
            .into_iter()
            .map(parse_seat_information)
            .max_by_key(|seat| seat.id)
        {
            return seat.id;
        }
        0
    }
}

mod puzzle_2 {
    use super::*;

    pub fn find_missing_seat() -> u32 {
        let mut input: Vec<u32> = get_input_from_file()
            .into_iter()
            .map(|s| parse_seat_information(s).id)
            .collect();
        input.sort_unstable();
        let mut missing_id = 0;
        for seats in input.windows(2).skip(1) {
            if seats.len() != 2 {
                continue;
            };
            if seats[1] - 1 != seats[0] {
                missing_id = seats[0] + 1;
                break;
            }
        }
        missing_id
    }
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
        assert_eq!(parse_seat_information(input.to_string()), expected)
    }

    #[test]
    fn day_5_puzzle_1_sol() {
        assert_eq!(puzzle_1::find_highest_id(), 885);
    }

    #[test]
    fn day_5_puzzle_2_sol() {
        assert_eq!(puzzle_2::find_missing_seat(), 623);
    }
}
