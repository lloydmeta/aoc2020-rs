use combine::lib::collections::HashMap;
use anyhow::Result;
use itertools::Itertools;

const INPUT: &str = include_str!("../data/day_10_input");

pub fn run() -> Result<()> {
    println!("*** Day 10: Adapter Array ***");
    println!("Input: {}", INPUT);
    let numbers = parse(INPUT);

    let differences_distribution = differences_between_consecutive_elements(&numbers);
    let solution_1 = differences_distribution.get(&1).zip(differences_distribution.get(&3)).map(|(one, three)| {
        *one * three
    });
    println!("Solution 1: {:?}", solution_1);

    Ok(())
}


fn parse(s: &str) -> Vec<usize> {
    s.split('\n').filter_map(|i| i.parse().ok()).collect()
}

fn differences_between_consecutive_elements(s: &[usize]) -> HashMap<usize, isize> {
    let mut sorted: Vec<_> = s.iter().copied().sorted().collect();

    // add 0 to the front (wall), and last + 3 to the back (device)
    if let Some(last) = sorted.last().copied() {
        sorted.push(last + 3);
    }

    let mut sorted_with_wall = vec![0];
    sorted_with_wall.append(&mut sorted);

    sorted_with_wall
        .iter()
        .zip(sorted_with_wall.iter().skip(1))
        .fold(HashMap::new(), |mut acc, (prev, next)| {
            let diff = *next - prev;
            *acc.entry(diff).or_insert(0) += 1;
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        let parsed = parse(input);
        assert_eq!(31, parsed.len())
    }

    #[test]
    fn differences_between_consecutive_elements_test_1() {
        let v = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let distribution = differences_between_consecutive_elements(&v);
        assert_eq!(7, *distribution.get(&1).unwrap());
        assert_eq!(5, *distribution.get(&3).unwrap());
    }

    #[test]
    fn differences_between_consecutive_elements_test_2() {
        let v = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        let distribution = differences_between_consecutive_elements(&v);
        assert_eq!(22, *distribution.get(&1).unwrap());
        assert_eq!(10, *distribution.get(&3).unwrap());
    }
}
