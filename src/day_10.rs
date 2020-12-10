use anyhow::Result;
use combine::lib::collections::HashMap;
use itertools::Itertools;

const INPUT: &str = include_str!("../data/day_10_input");

pub fn run() -> Result<()> {
    println!("*** Day 10: Adapter Array ***");
    println!("Input: {}", INPUT);
    let numbers = parse(INPUT);

    let differences_distribution = differences_between_consecutive_elements(&numbers);
    let solution_1 = differences_distribution
        .get(&1)
        .zip(differences_distribution.get(&3))
        .map(|(one, three)| *one * three);
    println!("Solution 1: {:?}", solution_1);

    println!("Solution 2: {:?}", count_viable_chains(&numbers));

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

fn count_viable_chains(adapters: &[usize]) -> usize {
    let sorted_adapter_joltages: Vec<_> = adapters.iter().sorted().copied().collect();

    if let Some(&last_adapter_joltage) = sorted_adapter_joltages.last() {
        let times_joltage_appears_in_chains = sorted_adapter_joltages.into_iter().fold(
            // Lookup that maps adapter joltage to number of times it can appear in a chain
            // given the adapters that we have
            HashMap::new(),
            |mut times_joltage_appears_in_chains, adapter_joltage| {
                if adapter_joltage <= 3 {
                    *times_joltage_appears_in_chains
                        .entry(adapter_joltage)
                        .or_insert(0) += 1;
                }

                (1..4.min(adapter_joltage)).into_iter().fold(
                    times_joltage_appears_in_chains,
                    |mut inner_times_joltage_appears_in_chains, joltage_decrease| {
                        let lower_joltage = adapter_joltage - joltage_decrease;
                        let times_lower_joltage_has_appeared =
                            inner_times_joltage_appears_in_chains
                                .get(&lower_joltage)
                                .copied()
                                .unwrap_or(0);
                        *inner_times_joltage_appears_in_chains
                            .entry(adapter_joltage)
                            .or_insert(0) += times_lower_joltage_has_appeared;
                        inner_times_joltage_appears_in_chains
                    },
                )
            },
        );

        // the last adapter *must* be part of the chain because the device adapter joltage is
        // based on it.
        times_joltage_appears_in_chains
            .get(&last_adapter_joltage)
            .copied()
            .unwrap_or(0)
    } else {
        0
    }
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

    #[test]
    fn count_viable_chains_test_1() {
        let v = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let r = count_viable_chains(&v);
        assert_eq!(8, r);
        // panic!()
    }

    #[test]
    fn count_viable_chains_test_2() {
        let v = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        let r = count_viable_chains(&v);
        assert_eq!(19208, r)
    }
}
