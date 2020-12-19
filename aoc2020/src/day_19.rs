use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub enum Rule {
    Letter(char),
    Ptr(HashSet<Vec<usize>>),
    Null,
}

impl Rule {
    pub fn new(s: &str) -> Rule {
        if s.contains('"') {
            let c = s
                .replace('"', "")
                .chars()
                .next()
                .expect("valid letter rule should only be 1 char");
            return Self::Letter(c);
        }

        let sub_rules: HashSet<_> = s
            .split(" | ")
            .map(|rules| {
                rules
                    .trim()
                    .split_whitespace()
                    .filter_map(|r| r.parse::<usize>().ok())
                    .collect::<Vec<_>>()
            })
            .collect();
        Self::Ptr(sub_rules)
    }
}

#[aoc_generator(day19)]
pub fn generator(input: &str) -> (Vec<Rule>, Vec<String>) {
    let mut rules = vec![Rule::Null; 150];
    let mut iter = input.lines();
    while let Some(line) = iter.next() {
        if line.is_empty() {
            break;
        }
        let mut split = line.split(": ");
        let rule_num = split
            .next()
            .expect("rule number should be there")
            .parse::<usize>()
            .expect("rule number should be a number");

        let rule = Rule::new(split.next().expect("rules should exist"));

        rules[rule_num] = rule;
    }
    (rules, iter.map(|s| s.to_string()).collect())
}

#[aoc(day19, part1)]
pub fn puzzle_1((rules, messages): &(Vec<Rule>, Vec<String>)) -> usize {
    let valid_messages: HashSet<String> = generate_valid_messages(rules, Vec::new(), 0)
        .into_iter()
        .map(|s| s.into_iter().collect::<String>())
        .collect();
    for msg in &valid_messages {
        println!("{:?}", msg);
    }

    messages
        .iter()
        .filter(|m| valid_messages.get(*m).is_some())
        .count()
}

pub fn generate_valid_messages(
    rules: &[Rule],
    mut permutations: Vec<Vec<char>>,
    starting_rule: usize,
) -> Vec<Vec<char>> {
    match &rules[starting_rule] {
        Rule::Letter(c) => {
            for p in permutations.iter_mut() {
                p.push(*c);
            }
            if permutations.is_empty() {
                permutations.push(vec![*c]);
            }
            return permutations;
        }
        Rule::Ptr(r) => {
            let mut temp = Vec::new();
            for rule_set in r {
                let mut local_rules = permutations.clone();
                for rule in rule_set {
                    local_rules = generate_valid_messages(rules, local_rules, *rule);
                }
                temp.append(&mut local_rules);
            }
            permutations = temp;
        }
        Rule::Null => {
            eprintln!("Rule doesn't exist");
        }
    }
    permutations
}

#[aoc(day19, part2)]
pub fn puzzle_2((rules, messages): &(Vec<Rule>, Vec<String>)) -> usize {
    let mut rules = rules.to_owned();
    rules[8] = Rule::Ptr(vec![vec![42], vec![42, 8]].into_iter().collect());
    rules[11] = Rule::Ptr(vec![vec![42, 31], vec![42, 11, 31]].into_iter().collect());
    let mut cache = HashMap::new();

    messages
        .iter()
        .filter(|m| matches(&rules, 0, m, &mut cache))
        .count()
}

pub fn matches(
    rules: &[Rule],
    rule: usize,
    string: &str,
    cache: &mut HashMap<(usize, String), bool>,
) -> bool {
    if let Some(&result) = cache.get(&(rule, string.to_string())) {
        return result;
    }
    let result = match &rules[rule] {
        Rule::Letter(c) => c.to_string() == string,
        Rule::Ptr(sub_rules) => sub_rules
            .iter()
            .any(|sub_rule| matches_sub_rule(rules, sub_rule, string, cache)),
        Rule::Null => {
            eprintln!("Some weird error");
            false
        }
    };

    cache.insert((rule, string.to_string()), result);
    result
}

pub fn matches_sub_rule(
    rules: &[Rule],
    sub_rule: &[usize],
    string: &str,
    cache: &mut HashMap<(usize, String), bool>,
) -> bool {
    if string.is_empty() && sub_rule.is_empty() {
        return true;
    }
    if sub_rule.is_empty() {
        return false;
    }

    let rule = sub_rule[0];
    for r in 1..=string.len() {
        if matches(rules, rule, &string[..r], cache)
            && matches_sub_rule(rules, &sub_rule[1..], &string[r..], cache)
        {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input_from_file;

    const TEST_INPUT: &str = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";

    #[test]
    fn puzzle_1_test() {
        let input = generator(TEST_INPUT);
        assert_eq!(puzzle_1(&input), 2);
    }

    #[test]
    fn puzzle_1_sol() {
        let input = get_input_from_file("input/2020/day19.txt");
        let input = generator(&input);
        assert_eq!(puzzle_1(&input), 160);
    }

    const TEST_INPUT_2: &str = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";

    #[test]
    fn puzzle_2_test() {
        let input = generator(TEST_INPUT_2);
        assert_eq!(puzzle_2(&input), 12);
    }

    #[test]
    fn puzzle_2_sol() {
        let input = get_input_from_file("input/2020/day19.txt");
        let input = generator(&input);
        assert_eq!(puzzle_2(&input), 357);
    }
}
