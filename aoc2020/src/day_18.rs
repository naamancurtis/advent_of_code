pub enum MathOp {
    Add,
    Multiply,
    Null,
}

#[aoc_generator(day18)]
pub fn generator(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.replace(" ", "").chars().collect())
        .collect()
}

#[aoc(day18, part1)]
pub fn puzzle_1(input: &[Vec<char>]) -> u64 {
    input.iter().map(|eq| do_fake_math_1(eq)).sum()
}

pub fn do_fake_math_1(equation: &[char]) -> u64 {
    let mut ptr = 0;
    let mut sum = 0;
    let mut current_op = MathOp::Null;
    while ptr < equation.len() {
        match equation[ptr] {
            '(' => {
                let mut paren_counter = 0;
                let mut temp_ptr = ptr + 1;
                loop {
                    match equation[temp_ptr] {
                        '(' => paren_counter += 1,
                        ')' => {
                            if paren_counter == 0 {
                                break;
                            }
                            paren_counter -= 1
                        }
                        _ => {}
                    }
                    temp_ptr += 1;
                }
                match current_op {
                    MathOp::Add => sum += do_fake_math_1(&equation[ptr + 1..temp_ptr]),
                    MathOp::Multiply => sum *= do_fake_math_1(&equation[ptr + 1..temp_ptr]),
                    MathOp::Null => sum = do_fake_math_1(&equation[ptr + 1..temp_ptr]),
                };
                ptr = temp_ptr;
            }
            '+' => current_op = MathOp::Add,
            '*' => current_op = MathOp::Multiply,
            n if n.is_digit(10) => {
                let n = n.to_digit(10).expect("already checked it is a digit") as u64;
                match current_op {
                    MathOp::Add => sum += n,
                    MathOp::Multiply => sum *= n,
                    MathOp::Null => sum = n,
                };
            }
            ')' => {}
            err => panic!("got {}, however didn't expect to get anything", err),
        }

        ptr += 1;
    }
    sum
}

#[aoc(day18, part2)]
pub fn puzzle_2(input: &[Vec<char>]) -> u64 {
    let mut input: Vec<Vec<String>> = input
        .iter()
        .map(|eq| eq.iter().map(|c| c.to_string()).collect())
        .collect();
    input.iter_mut().map(|eq| do_fake_math_2(eq)).sum()
}

pub fn do_fake_math_2(equation: &mut [String]) -> u64 {
    let mut equation = equation.to_owned();
    let mut ptr = 0;

    // Resolve all parens
    while ptr < equation.len() {
        if equation[ptr].as_str() == "(" {
            let mut paren_counter = 0;
            let mut temp_ptr = ptr + 1;
            loop {
                match equation[temp_ptr].as_str() {
                    "(" => paren_counter += 1,
                    ")" => {
                        if paren_counter == 0 {
                            break;
                        }
                        paren_counter -= 1
                    }
                    _ => {}
                }
                temp_ptr += 1;
            }
            let result = do_fake_math_2(&mut equation[ptr + 1..temp_ptr]).to_string();
            let inner_sum = vec![result];
            equation.splice(ptr..=temp_ptr, inner_sum);
        }
        ptr += 1;
    }
    ptr = 0;

    // Resolve all addition
    while ptr < equation.len() {
        if equation[ptr].as_str() == "+" {
            let num_1 = equation[ptr - 1]
                .parse::<u64>()
                .expect("should be valid number");
            let num_2 = equation[ptr + 1]
                .parse::<u64>()
                .expect("should be valid number");
            let inner_sum = vec![(num_1 + num_2).to_string()];
            equation.splice(ptr - 1..=ptr + 1, inner_sum);

            // As we're mutating the array in place, [ptr - 1, ptr, ptr + 1]
            // we need to reset the value of the `ptr` back to `ptr - 1`
            ptr -= 1;
        } else {
            // as we're
            ptr += 1;
        }
    }

    // Resolve all multiplication
    equation
        .iter()
        .filter_map(|n| n.parse::<u64>().ok())
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input_from_file;

    const TEST_INPUT: &str = "2 * 3 + (4 * 5)
    5 + (8 * 3 + 9 + 3 * 4 * 3)
    5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
    ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

    #[test]
    fn puzzle_1_test() {
        let input = generator(TEST_INPUT);
        assert_eq!(puzzle_1(&input), 26 + 437 + 12240 + 13632);
    }

    #[test]
    fn puzzle_1_sol() {
        let input = get_input_from_file("input/2020/day18.txt");
        let input = generator(&input);
        assert_eq!(puzzle_1(&input), 701339185745);
    }

    #[test]
    fn puzzle_2_test() {
        let input = generator(TEST_INPUT);
        assert_eq!(puzzle_2(&input), 46 + 1445 + 669060 + 23340);
    }

    #[test]
    fn puzzle_2_sol() {
        let input = get_input_from_file("input/2020/day18.txt");
        let input = generator(&input);
        assert_eq!(puzzle_2(&input), 4208490449905);
    }
}
