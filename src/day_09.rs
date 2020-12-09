use anyhow::Result;
use itertools::Itertools;

const INPUT: &str = include_str!("../data/day_09_input");

pub fn run() -> Result<()> {
    println!("*** Day 9: Encoding Error ***");
    println!("Input: {}", INPUT);
    let numbers = parse(INPUT);

    let solution_1 = find_preamble_error(&numbers, 25);
    println!("Solution 1: {:?}", solution_1);

    let solution_2 = find_min_max_sums_in_first_window_adding_to(&numbers, solution_1.unwrap_or(0));
    println!("Solution 2: {:?}", solution_2);
    Ok(())
}

fn find_preamble_error(v: &[usize], preamble_size: usize) -> Option<usize> {
    v.windows(preamble_size + 1)
        .into_iter()
        .filter_map(|slice| {
            if let Some(number) = slice.last() {
                let preamble = slice.iter().take(preamble_size).copied();
                let first_sum_in_preamble_to_num = preamble
                    .combinations(2)
                    .find(|v| v.iter().sum::<usize>() == *number);
                if first_sum_in_preamble_to_num.is_none() {
                    // nothing in the preamble adds to this number
                    Some(*number)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .next()
}

fn find_min_max_sums_in_first_window_adding_to(v: &[usize], target: usize) -> Option<usize> {
    let valid_numbers: Vec<_> = v.iter().filter(|i| **i <= target).copied().collect();
    (2..valid_numbers.len())
        .into_iter()
        .flat_map(|window_size| {
            let sums_to_target: Vec<_> = valid_numbers
                .as_slice()
                .windows(window_size)
                .filter_map(|window| {
                    let sum = window.iter().sum::<usize>();
                    if sum == target {
                        window
                            .iter()
                            .min()
                            .zip(window.iter().max())
                            .map(|(min, max)| *min + max)
                    } else {
                        None
                    }
                })
                .collect();
            sums_to_target
        })
        .next()
}

fn parse(s: &str) -> Vec<usize> {
    s.split('\n').filter_map(|s| s.parse().ok()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        let r = parse(input);

        let expected = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(expected, r);
    }

    #[test]
    fn find_preamble_error_test() {
        let v = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        assert_eq!(Some(&127), find_preamble_error(&v, 5).first());
    }

    #[test]
    fn find_windows_adding_to_test() {
        let v = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        let windows_adding_to_127 = find_min_max_sums_in_first_window_adding_to(&v, 127);
        assert_eq!(Some(62), windows_adding_to_127);
    }
}
