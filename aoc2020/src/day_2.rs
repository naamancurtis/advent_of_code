use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX: Regex = Regex::new(
        r"(?P<min>\d+)-(?P<max>\d+)\s(?P<letter>[[:alpha:]]):\s(?P<password>[[:alpha:]]+)"
    )
    .expect("regex should be fine");
}

pub struct PasswordDetails {
    pub min: i8,
    pub max: i8,
    pub letter: char,
    pub password: String,
}

impl PasswordDetails {
    fn is_valid(&self) -> bool {
        let mut counter = 0;
        for c in self.password.chars() {
            if c == self.letter {
                counter += 1;
                if counter > self.max {
                    return false;
                }
            }
        }
        if counter < self.min {
            return false;
        }
        true
    }

    fn is_valid_2(&self) -> bool {
        let first = self.password.chars().nth((self.min - 1) as usize).unwrap();
        let last = self.password.chars().nth((self.max - 1) as usize).unwrap();
        if (first == self.letter || last == self.letter) && first != last {
            return true;
        }
        false
    }
}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<PasswordDetails> {
    input
        .lines()
        .filter_map(|l| {
            let captures = REGEX.captures(l)?;
            let min: i8 = captures.name("min")?.as_str().parse().ok()?;
            let max: i8 = captures.name("max")?.as_str().parse().ok()?;
            let letter = captures.name("letter")?.as_str().chars().next()?;
            let password = captures.name("password")?.as_str().to_string();

            Some({
                PasswordDetails {
                    min,
                    max,
                    letter,
                    password,
                }
            })
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn count_valid_passwords(input: &[PasswordDetails]) -> u32 {
    input.iter().filter(|p| p.is_valid()).count() as u32
}

#[aoc(day2, part2)]
pub fn count_valid_passwords_2(input: &[PasswordDetails]) -> u32 {
    input.iter().filter(|p| p.is_valid_2()).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_1_test() {
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        let input = generator(input);
        assert_eq!(count_valid_passwords(&input), 2);
    }

    #[test]
    fn puzzle_2_test() {
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        let input = generator(input);
        assert_eq!(count_valid_passwords(&input), 2);
        assert_eq!(count_valid_passwords_2(&input), 1);
    }
}
