#![allow(dead_code)]
use std::error::Error;
use std::io::{BufRead, BufReader, Read};

// Could probably do the validation inline as you're reading the file (allowing
// you to jump to the next passport if the one you're already reading is invalid), but
// that approach is probably overly complex for this problem

pub fn validate_passports<R: Read>(
    read: R,
    should_validate_passports: bool,
) -> Result<u32, Box<dyn Error>> {
    let reader = BufReader::new(read);

    // Read the file and collect the various lines together into passports, each
    // entry in the vec is 1 passport
    let (values, _) = reader.lines().into_iter().fold(
        (Vec::default(), false),
        |(mut v, create_new_entry), line| {
            if let Ok(line) = line {
                let line = line.trim().to_string();
                if line.is_empty() {
                    return (v, true);
                }
                if create_new_entry {
                    v.push(line);
                    return (v, false);
                }
                if let Some(last) = v.last_mut() {
                    last.push(' ');
                    last.push_str(&line);
                    return (v, false);
                }
            }
            (v, create_new_entry)
        },
    );
    let count = values
        .into_iter()
        .filter_map(|v| {
            let passport_key_values: Vec<(String, String)> = v
                .split(' ')
                .map(|entry| {
                    let mut key_value = entry.split(':').map(|v| v.to_string());
                    let first = key_value.next().unwrap_or_default();
                    let second = key_value.next().unwrap_or_default();
                    (first, second)
                })
                .collect();

            let mut passport_key_count = passport_key_values.len();
            if should_validate_passports {
                passport_key_count = passport_key_values
                    .iter()
                    .filter(|v| passport_entry_validator(v))
                    .count();
            }

            return match passport_key_count {
                8 => Some(passport_key_values),
                7 if !passport_key_values.iter().any(|key| &key.0 == "cid") => {
                    Some(passport_key_values)
                }
                _ => None,
            };
        })
        .count();
    Ok(count as u32)
}

fn passport_entry_validator((key, value): &(String, String)) -> bool {
    match key.as_str() {
        "byr" => date_validator(value, 1920, 2002),
        "iyr" => date_validator(value, 2010, 2020),
        "eyr" => date_validator(value, 2020, 2030),
        "hgt" => {
            let mut height = String::new();
            let mut metric = String::new();
            value.chars().for_each(|c| {
                if c.is_digit(10) {
                    height.push(c);
                } else {
                    metric.push(c);
                }
            });
            if let Ok(height) = height.parse::<u32>() {
                if &metric == "cm" && height >= 150 && height <= 193 {
                    return true;
                }
                if &metric == "in" && height >= 59 && height <= 76 {
                    return true;
                }
            }
            false
        }
        "hcl" => {
            let mut iter = value.chars();
            if iter.next() != Some('#') {
                return false;
            }
            let count = iter.filter(|c| c.is_digit(16)).count();
            if count != 6 {
                return false;
            }
            true
        }
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value.as_str()),
        "pid" => value.chars().filter(|c| c.is_digit(10)).count() == 9,
        "cid" => true,
        _ => false,
    }
}

fn date_validator(date: &str, min: u32, max: u32) -> bool {
    if date.len() != 4 {
        return false;
    }
    if let Ok(num) = date.parse::<u32>() {
        if num >= min && num <= max {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn puzzle_1_test() {
        let input = r#"
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
            "#;
        let result = validate_passports(input.as_bytes(), false).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn puzzle_1_sol() {
        let file = File::open("./data/day_4.txt").expect("file failed to open");
        let result = validate_passports(file, false).unwrap();
        assert_eq!(result, 228);
    }

    #[test]
    fn puzzle_2_sol() {
        let file = File::open("./data/day_4.txt").expect("file failed to open");
        let result = validate_passports(file, true).unwrap();
        assert_eq!(result, 175);
    }
}
