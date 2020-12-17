use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

lazy_static! {
    pub static ref REGEX: Regex = Regex::new(r"(.+):\s([0-9]+)-([0-9]+)\sor\s([0-9]+)-([0-9]+)")
        .expect("regex should be fine");
}

#[derive(Debug, Default)]
pub struct Input {
    pub rules: HashMap<String, Vec<usize>>,
    pub ticket: Vec<usize>,
    pub nearby_tickets: Vec<Vec<usize>>,
}

#[aoc_generator(day16)]
pub fn generator(input: &str) -> Input {
    let mut new_line_counter = 0;
    let mut data = Input::default();
    for line in input.lines() {
        if line.is_empty() || (new_line_counter % 2 != 0 && new_line_counter != 0) {
            new_line_counter += 1;
            continue;
        }

        if new_line_counter == 0 {
            let captures = REGEX.captures(line).expect("regex should match");
            let mut cap = captures.iter();
            cap.next(); // throw away the whole match
            let field = cap
                .next()
                .flatten()
                .expect("there should be a title")
                .as_str();
            let rules: Vec<usize> = cap
                .filter_map(|num| num?.as_str().parse::<usize>().ok())
                .collect();
            data.rules.insert(field.to_string(), rules);
            continue;
        }

        if new_line_counter == 2 {
            data.ticket = parse_ticket(line);
            continue;
        }
        data.nearby_tickets.push(parse_ticket(line));
    }
    data
}

pub fn parse_ticket(input: &str) -> Vec<usize> {
    input
        .split(',')
        .filter_map(|v| v.parse::<usize>().ok())
        .collect()
}

#[aoc(day16, part1)]
pub fn puzzle_1(input: &Input) -> usize {
    let rules = input.rules.values();
    let consolidated_rules = generate_consolidated_rules(rules);

    let mut error_rate = 0;
    for ticket in &input.nearby_tickets {
        for num in ticket {
            if !consolidated_rules[*num] {
                error_rate += *num;
            }
        }
    }
    error_rate
}

pub fn generate_consolidated_rules<'a>(rules: impl Iterator<Item = &'a Vec<usize>>) -> Vec<bool> {
    let mut consolidated_rules = vec![false; 1000];
    for rule in rules {
        let mut idx = 0;
        while idx < rule.len() {
            for i in rule[idx]..=rule[idx + 1] {
                if i > consolidated_rules.len() {
                    let mut appender = vec![false; (i + 1) - consolidated_rules.len()];
                    consolidated_rules.append(&mut appender);
                }
                consolidated_rules[i] = true;
            }
            idx += 2;
        }
    }
    consolidated_rules
}

#[aoc(day16, part2)]
pub fn puzzle_2(input: &Input) -> usize {
    let rules = input.rules.values();
    let len = rules.len();
    let consolidated_rules = generate_consolidated_rules(rules);

    let fields: HashSet<&String> = input.rules.keys().collect();

    let valid_tickets: Vec<&Vec<usize>> = input
        .nearby_tickets
        .iter()
        .filter(|ticket| {
            !ticket
                .iter()
                .map(|field| &consolidated_rules[*field])
                .any(|field| !*field)
        })
        .collect();

    let mut candidates = vec![fields; input.ticket.len()];

    while candidates
        .iter()
        .map(|candidate| candidate.len())
        .sum::<usize>()
        != len
    {
        for i in 0..candidates.len() {
            let mut fields_to_remove = Vec::new();
            'fields: for field in candidates[i].iter() {
                let rule = input
                    .rules
                    .get(*field)
                    .expect("keys should always be valid");
                for valid_ticket in &valid_tickets {
                    let val = valid_ticket[i];
                    // This is invalid
                    if val < rule[0] || (val > rule[1] && val < rule[2]) || val > rule[3] {
                        fields_to_remove.push(*field);
                        continue 'fields;
                    }
                }
            }
            for field in fields_to_remove.into_iter() {
                candidates[i].remove(field);
            }

            // If we only have one field, we don't have to check this anymore on
            // other candidates as fields aren't repeated
            if candidates[i].len() == 1 {
                let field = (**(candidates[i].iter().next().expect("we know len is 1"))).clone();
                for (j, c) in candidates.iter_mut().enumerate() {
                    if i != j {
                        c.remove(&field);
                    }
                }
            }
        }
    }
    candidates
        .into_iter()
        .filter_map(|set| set.iter().next().map(|s| s.to_owned()))
        .enumerate()
        .filter_map(|(i, field)| {
            if field.contains("departure") {
                return Some(input.ticket[i]);
            }
            None
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input_from_file;

    const TEST_INPUT: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test]
    fn puzzle_1_test() {
        let input = generator(TEST_INPUT);
        assert_eq!(puzzle_1(&input), 71);
    }

    #[test]
    fn puzzle_1_sol() {
        let input = get_input_from_file("input/2020/day16.txt");
        let input = generator(&input);
        assert_eq!(puzzle_1(&input), 25972);
    }

    #[test]
    fn puzzle_2_sol() {
        let input = get_input_from_file("input/2020/day16.txt");
        let input = generator(&input);
        assert_eq!(puzzle_2(&input), 622670335901);
    }
}
