#[aoc_generator(day13)]
pub fn generator(input: &str) -> (i32, Vec<Option<i32>>) {
    let mut lines = input.lines();
    let arrival_time: i32 = lines
        .next()
        .expect("should contain valid time")
        .parse()
        .expect("arrival time should be a number");
    let bus_ids = lines
        .next()
        .expect("second line should be there")
        .split(',')
        .map(|n| n.parse::<i32>().ok())
        .collect();
    (arrival_time, bus_ids)
}

#[aoc(day13, part1)]
pub fn puzzle_1((arrival_time, bus_ids): &(i32, Vec<Option<i32>>)) -> i32 {
    let mut min = std::i32::MAX;
    let mut min_bus_id = None;
    for bus_id in bus_ids.iter().filter(|id| id.is_some()) {
        if let Some(id) = bus_id {
            let time_to_wait = id - (arrival_time % id);
            if time_to_wait < min {
                min = time_to_wait;
                min_bus_id = bus_id.as_ref();
            }
        }
    }
    min * min_bus_id.unwrap()
}

#[aoc(day13, part2)]
pub fn puzzle_2((_, bus_ids): &(i32, Vec<Option<i32>>)) -> u64 {
    bus_ids
        .iter()
        .enumerate()
        .fold(
            (0, 1),
            |(mut solution_so_far, product_so_far), (idx, bus_id)| {
                if let Some(bus_id) = bus_id {
                    let id = *bus_id as usize;
                    while (solution_so_far + idx) % id != 0 {
                        solution_so_far += product_so_far;
                    }
                    return (solution_so_far, product_so_far * id);
                }
                (solution_so_far, product_so_far)
            },
        )
        .0 as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn puzzle_1_test() {
        let input = generator(TEST_INPUT);
        assert_eq!(puzzle_1(&input), 295);
    }

    #[test]
    fn puzzle_2_test() {
        let input = generator(TEST_INPUT);
        assert_eq!(puzzle_2(&input), 1068781);
    }
}
