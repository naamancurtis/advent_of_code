mod puzzle_1 {
    fn solution(input: Vec<String>) -> u32 {
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

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::get_input_from_file;

        #[test]
        fn puzzle_1_works() {
            let input: Vec<String> = vec![
                "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b",
            ]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
            assert_eq!(solution(input), 11);
        }

        #[test]
        fn puzzle_1_sol() {
            let input = get_input_from_file("./data/day_6.txt");
            assert_eq!(solution(input), 6534);
        }
    }
}

mod puzzle_2 {
    fn solution(input: Vec<String>) -> u32 {
        let mut result = 0;
        let mut buffer = [0u8; 26];
        let mut people_in_group: u8 = 0;
        for line in input {
            // New group
            if line.is_empty() {
                result += buffer
                    .iter()
                    .filter_map(|count| {
                        if *count == people_in_group {
                            Some(())
                        } else {
                            None
                        }
                    })
                    .count();
                buffer = [0u8; 26];
                people_in_group = 0;
                continue;
            }
            people_in_group += 1;
            for c in line.chars() {
                let i = (c as u32 - 97) as usize;
                buffer[i] += 1;
            }
        }
        result += buffer
            .iter()
            .filter_map(|count| {
                if *count == people_in_group {
                    Some(())
                } else {
                    None
                }
            })
            .count();
        result as u32
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::get_input_from_file;

        #[test]
        fn puzzle_2_works() {
            let input: Vec<String> = vec![
                "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b",
            ]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
            assert_eq!(solution(input), 6);
        }

        #[test]
        fn puzzle_5_sol() {
            let input = get_input_from_file("./data/day_6.txt");
            assert_eq!(solution(input), 3402);
        }
    }
}
