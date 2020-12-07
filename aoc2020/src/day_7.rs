use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref REGEX: Regex =
        Regex::new(r"(\d+)\s?(\w+\s\w+) bags?").expect("regex should be fine");
}

pub struct Solution {
    puzzle_1: i32,
    puzzle_2: i64,
}

/// Bags that contain `no bags` will not be added to this map
///
/// Types:
///     - `String` - The _parent_ bag type
///     - `Vec<(i64, String)` - List of nested bag types `String` and the number `(i64)` of bags
///     of that type that must be within the parent bag
fn generate_map(input: &[String]) -> HashMap<String, Vec<(i64, String)>> {
    let mut map = HashMap::new();
    for line in input {
        let mut split = line.split("bags contain");
        if let Some(bag_type) = split.next() {
            if let Some(nested_bags) = split.next() {
                let iter = REGEX.captures_iter(nested_bags).filter_map(|capture| {
                    let number_of_bags_required = capture.get(1)?.as_str();
                    let number_of_bags_required = number_of_bags_required
                        .parse::<i64>()
                        .expect("number should be valid number");
                    let bag_type = capture.get(2)?.as_str();
                    Some((number_of_bags_required, bag_type.to_string()))
                });

                let result: Vec<(i64, String)> = iter.collect();
                // if there are no bags nested in this bag type, don't add it to
                // the map
                if result.is_empty() {
                    continue;
                }
                map.insert(bag_type.trim().to_string(), result);
            }
        }
    }
    map
}

pub fn solution(input: &[String], target: &str) -> Solution {
    let map = generate_map(input);

    // (i64, bool)
    // `i64` - stores the total number of bags stored within that bag
    // `bool` - stores whether it's possible to store the target bag somewhere
    // within the nested bag structure
    let mut cache = HashMap::new();

    // The cache is populated while doing the counting, by the time the counting
    // is finished, we can easily just do a look up to find out how many bags the target bag contains
    let count = map
        .keys()
        .filter_map(|key| {
            if recurse_into_bag(&key, &map, &mut cache, target).1 {
                return Some(());
            }
            None
        })
        .count();

    Solution {
        puzzle_1: count as i32,
        puzzle_2: cache.get(target).unwrap().0,
    }
}

fn recurse_into_bag(
    key: &str,
    map: &HashMap<String, Vec<(i64, String)>>,
    cache: &mut HashMap<String, (i64, bool)>,
    target: &str,
) -> (i64, bool) {
    if let Some(value) = cache.get(key) {
        return *value;
    }

    let mut can_store_target_bag = false;
    let mut total_count = 0;

    // if the key is not in the map, then this type of bag does not contain any
    // other bags within it
    if let Some(child_keys) = map.get(key) {
        for (multiplier, child_key) in child_keys {
            let (count, can_store) = recurse_into_bag(child_key, map, cache, target);

            total_count += (multiplier * count) + multiplier;

            // If you wanted to specialise for puzzle one, you could break here
            // as if one of the child bags can store the target bag, you don't
            // need to check the rest of the children
            if can_store || child_key == target {
                can_store_target_bag = true;
            }
        }
    }

    let result = (total_count, can_store_target_bag);
    let insert = cache.insert(key.to_string(), result);
    assert!(insert.is_none());
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input_from_file;

    const TEST_INPUT_1: [&str; 9] = [
        "light red bags contain 1 bright white bag, 2 muted yellow bags.",
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
        "bright white bags contain 1 shiny gold bag.",
        "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
        "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
        "faded blue bags contain no other bags.",
        "dotted black bags contain no other bags.",
    ];

    const TEST_INPUT_2: [&str; 7] = [
        "shiny gold bags contain 2 dark red bags.",
        "dark red bags contain 2 dark orange bags.",
        "dark orange bags contain 2 dark yellow bags.",
        "dark yellow bags contain 2 dark green bags.",
        "dark green bags contain 2 dark blue bags.",
        "dark blue bags contain 2 dark violet bags.",
        "dark violet bags contain no other bags.",
    ];

    #[test]
    fn puzzle_1_test() {
        let input: Vec<String> = TEST_INPUT_1.iter().map(|s| s.to_string()).collect();
        assert_eq!(solution(&input, "shiny gold").puzzle_1, 4);
    }

    #[test]
    fn puzzle_1_sol() {
        let input = get_input_from_file("./data/day_7.txt");
        assert_eq!(solution(&input, "shiny gold").puzzle_1, 131);
    }

    #[test]
    fn puzzle_2_test() {
        let input: Vec<String> = TEST_INPUT_2.iter().map(|s| s.to_string()).collect();
        assert_eq!(solution(&input, "shiny gold").puzzle_2, 126);
    }

    #[test]
    fn puzzle_2_sol() {
        let input = get_input_from_file("./data/day_7.txt");
        assert_eq!(solution(&input, "shiny gold").puzzle_2, 11261);
    }
}
