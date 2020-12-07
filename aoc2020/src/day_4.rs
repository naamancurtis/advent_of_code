#[aoc_generator(day4)]
pub fn generator(input: &str) -> Vec<Vec<(String, String)>> {
    let (values, _) = input.lines().into_iter().fold(
        (Vec::default(), false),
        |(mut v, create_new_entry), line| {
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
            (v, create_new_entry)
        },
    );
    values
        .into_iter()
        .map(|v| {
            v.split(' ')
                .map(|entry| {
                    let mut key_value = entry.split(':').map(|v| v.to_string());
                    let first = key_value.next().unwrap_or_default();
                    let second = key_value.next().unwrap_or_default();
                    (first, second)
                })
                .collect()
        })
        .collect()
}

#[aoc(day4, part1)]
fn validate_passports(input: &[Vec<(String, String)>]) -> u32 {
    let count = input
        .iter()
        .filter_map(|passports| match passports.len() {
            8 => Some(()),
            7 if !passports.iter().any(|key| &key.0 == "cid") => Some(()),
            _ => None,
        })
        .count();
    count as u32
}

#[aoc(day4, part2)]
fn validate_passports_2(input: &[Vec<(String, String)>]) -> u32 {
    let count = input
        .iter()
        .filter_map(|passports| {
            match passports
                .iter()
                .filter(|p| passport_entry_validator(p))
                .count()
            {
                8 => Some(()),
                7 if !passports.iter().any(|key| &key.0 == "cid") => Some(()),
                _ => None,
            }
        })
        .count();
    count as u32
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
        let input = generator(input);
        let result = validate_passports(&input);
        assert_eq!(result, 2);
    }
}
