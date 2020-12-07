#[aoc_generator(day6)]
pub fn generator(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

#[aoc(day6, part1)]
fn solution_1(input: &[String]) -> u32 {
    let mut result = 0;
    let mut buffer = [false; 26];
    for line in input {
        // New group
        if line.is_empty() {
            result += buffer.iter().filter(|v| **v).count();
            buffer = [false; 26];
            continue;
        }
        for c in line.chars() {
            let i = (c as u32 - 97) as usize;
            buffer[i] = true;
        }
    }
    result += buffer.iter().filter(|v| **v).count();
    result as u32
}

#[aoc(day6, part2)]
fn solution_2(input: &[String]) -> u32 {
    let mut result = 0;
    let mut buffer = [0u8; 26];
    let mut people_in_group: u8 = 0;

    let count_people = |buffer: [u8; 26], people_in_group: u8| {
        buffer
            .iter()
            .filter_map(|count| {
                if *count == people_in_group {
                    Some(())
                } else {
                    None
                }
            })
            .count()
    };

    for line in input {
        // New group
        if line.is_empty() {
            result += count_people(buffer, people_in_group);
            people_in_group = 0;
            buffer = [0u8; 26];
            continue;
        }
        people_in_group += 1;
        for c in line.chars() {
            let i = (c as u32 - 97) as usize;
            buffer[i] += 1;
        }
    }
    result += count_people(buffer, people_in_group);
    result as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_1_works() {
        let input: Vec<String> = vec![
            "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(solution_1(&input), 11);
    }
    #[test]
    fn puzzle_2_works() {
        let input: Vec<String> = vec![
            "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(solution_2(&input), 6);
    }
}
