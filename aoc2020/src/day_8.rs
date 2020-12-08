#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

impl Operation {
    pub fn new(input: &str) -> Option<Operation> {
        let mut iter = input.split_whitespace();
        let op = iter.next()?;
        let val = iter.next()?;
        let val: isize = val.parse().ok()?;

        Some(match op {
            "nop" => Operation::Nop(val),
            "jmp" => Operation::Jmp(val),
            "acc" => Operation::Acc(val),
            _ => panic!("invalid operation"),
        })
    }
}

#[aoc_generator(day8)]
pub fn generator(input: &str) -> Vec<Operation> {
    input.lines().filter_map(|l| Operation::new(l)).collect()
}

#[aoc(day8, part1)]
pub fn puzzle_1(input: &[Operation]) -> i32 {
    let mut ptr = 0;
    let mut acc = 0;
    let mut freq = vec![false; input.len()];
    loop {
        if freq[ptr] {
            return acc;
        }
        freq[ptr] = true;
        match input[ptr] {
            Operation::Nop(_) => {
                ptr += 1;
            }
            Operation::Acc(val) => {
                acc += val as i32;
                ptr += 1;
            }
            Operation::Jmp(to) => {
                let temp_ptr = (ptr as isize + to).rem_euclid(input.len() as isize);
                ptr = temp_ptr as usize;
            }
        };
    }
}

#[aoc(day8, part2)]
pub fn puzzle_2(input: &[Operation]) -> i32 {
    let mut input = input.to_owned();
    let mut ptr = 0;
    let mut acc = 0;

    loop {
        if ptr == input.len() {
            return acc;
        }
        if let Some(mut op) = match input[ptr] {
            Operation::Jmp(val) => Some(Operation::Nop(val)),
            Operation::Nop(val) => Some(Operation::Jmp(val)),
            _ => None,
        } {
            std::mem::swap(&mut op, &mut input[ptr]);
            if let Some(val) = run_scenario(&input, ptr, acc) {
                return val;
            }
            std::mem::swap(&mut op, &mut input[ptr]);
        }
        let results = match_op(&input, ptr, acc);
        ptr = results.0;
        acc = results.1;
    }
}

pub fn run_scenario(input: &[Operation], mut ptr: usize, mut acc: i32) -> Option<i32> {
    let mut freq = vec![false; input.len()];
    loop {
        freq[ptr] = true;
        let results = match_op(input, ptr, acc);
        ptr = results.0;
        acc = results.1;
        if ptr == input.len() {
            return Some(acc);
        }
        if ptr > input.len() || freq[ptr] {
            return None;
        }
    }
}

fn match_op(input: &[Operation], ptr: usize, acc: i32) -> (usize, i32) {
    match input[ptr] {
        Operation::Nop(_) => (ptr + 1, acc),
        Operation::Jmp(to) => {
            let temp_ptr = ptr as isize + to;
            (temp_ptr as usize, acc)
        }
        Operation::Acc(val) => (ptr + 1, acc + val as i32),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
nop +0\n
acc +1\n
jmp +4\n
acc +3\n
jmp -3\n
acc -99\n
acc +1\n
jmp -4\n
acc +6\n
        ";

    #[test]
    fn puzzle_1_test() {
        let input = generator(TEST_INPUT);
        assert_eq!(puzzle_1(&input), 5);
    }

    #[test]
    fn puzzle_2_test() {
        let input = generator(TEST_INPUT);
        assert_eq!(puzzle_2(&input), 8);
    }
}
