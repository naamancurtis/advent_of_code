use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref REGEX: Regex =
        Regex::new(r"(\d+)\s?(\w+\s\w+) bags?").expect("regex should be fine");
}

const TARGET: &str = "shiny gold";

/// Bags that contain `no bags` will not be added to this map
///
/// Types:
///     - `String` - The _parent_ bag type
///     - `Vec<(i64, String)` - List of nested bag types `String` and the number `(i64)` of bags
///     of that type that must be within the parent bag
#[aoc_generator(day7)]
pub fn generate_map(input: &str) -> HashMap<String, Vec<(i64, String)>> {
    let mut map = HashMap::new();
    for line in input.lines() {
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

#[aoc(day7, part1)]
pub fn solution(map: &HashMap<String, Vec<(i64, String)>>) -> i32 {
    // (i64, bool)
    // `i64` - stores the total number of bags stored within that bag
    // `bool` - stores whether it's possible to store the target bag somewhere
    // within the nested bag structure
    let mut cache = HashMap::new();

    // The cache is populated while doing the counting, by the time the counting
    // is finished, we can easily just do a look up to find out how many bags the target bag contains
    map.keys()
        .filter_map(|key| {
            if recurse_into_bag(&key, &map, &mut cache, false).1 {
                return Some(());
            }
            None
        })
        .count() as i32
}

#[aoc(day7, part2)]
pub fn solution_2(map: &HashMap<String, Vec<(i64, String)>>) -> i64 {
    let mut cache = HashMap::new();
    recurse_into_bag(TARGET, &map, &mut cache, true).0
}

fn recurse_into_bag(
    key: &str,
    map: &HashMap<String, Vec<(i64, String)>>,
    cache: &mut HashMap<String, (i64, bool)>,
    solution_2: bool,
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
            let (count, can_store) = recurse_into_bag(child_key, map, cache, solution_2);

            total_count += (multiplier * count) + multiplier;

            // If you wanted to specialise for puzzle one, you could break here
            // as if one of the child bags can store the target bag, you don't
            // need to check the rest of the children
            if can_store || child_key == TARGET {
                can_store_target_bag = true;
                if !solution_2 {
                    break;
                }
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

    const TEST_INPUT_1: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.\n
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n
        bright white bags contain 1 shiny gold bag.\n
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n
        faded blue bags contain no other bags.\n
        dotted black bags contain no other bags.";

    const TEST_INPUT_2: &str = "shiny gold bags contain 2 dark red bags.\n
        dark red bags contain 2 dark orange bags.\n
        dark orange bags contain 2 dark yellow bags.\n
        dark yellow bags contain 2 dark green bags.\n
        dark green bags contain 2 dark blue bags.\n
        dark blue bags contain 2 dark violet bags.\n
        dark violet bags contain no other bags.";

    #[test]
    fn puzzle_1_test() {
        let input = generate_map(TEST_INPUT_1);
        assert_eq!(solution(&input), 4);
    }

    #[test]
    fn puzzle_2_test() {
        let input = generate_map(TEST_INPUT_2);
        assert_eq!(solution_2(&input), 126);
    }
}
