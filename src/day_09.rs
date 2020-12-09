use anyhow::Result;
use itertools::Itertools;

const INPUT: &str = include_str!("../data/day_09_input");

pub fn run() -> Result<()> {
    println!("*** Day 9: Encoding Error ***");
    println!("Input: {}", INPUT);
    let numbers = parse(INPUT);

    println!(
        "Solution 1: {:?}",
        find_preamble_error(&numbers, 25).first()
    );

    Ok(())
}

fn find_preamble_error(v: &[usize], preamble_size: usize) -> Vec<usize> {
    (preamble_size..v.len() - 1)
        .into_iter()
        .filter_map(|idx| {
            let number = v[idx];
            let preamble = v[idx - preamble_size..idx].into_iter().copied();
            let first_sum_in_preamble_to_num = preamble
                .combinations(2)
                .find(|v| v.iter().sum::<usize>() == number);
            if first_sum_in_preamble_to_num.is_none() {
                // nothing in the preamble adds to this number
                Some(number)
            } else {
                None
            }
        })
        .collect()
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
}
