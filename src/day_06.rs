use anyhow::Result;
use combine::parser::char::*;
use combine::*;

use combine::lib::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../data/day_06_input");

pub fn run() -> Result<()> {
    println!("*** Day 6: Custom Customs ***");
    println!("Input: {}", INPUT);
    // println!("Input: {}", INPUT);
    let groups_answers = parse(INPUT);

    println!(
        "Solution 1: {:?}",
        groups_answers.sum_of_group_distinct_answers()
    );

    println!(
        "Solution 2: {:?}",
        groups_answers.sum_of_group_same_answers()
    );

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct PersonAnswers(String);

#[derive(Debug, PartialEq, Eq)]
struct GroupAnswers(Vec<PersonAnswers>);

#[derive(Debug, PartialEq, Eq)]
struct GroupsAnswers(Vec<GroupAnswers>);

/// Would be more efficient to use bitwise ops ...
impl GroupsAnswers {
    fn sum_of_group_distinct_answers(&self) -> usize {
        self.0.iter().fold(0, |acc, GroupAnswers(group_answers)| {
            let group_distinct_answers =
                group_answers
                    .iter()
                    .fold(HashSet::with_capacity(26), |acc, PersonAnswers(s)| {
                        s.chars().into_iter().fold(acc, |mut acc, c| {
                            acc.insert(c);
                            acc
                        })
                    });
            acc + group_distinct_answers.len()
        })
    }

    fn sum_of_group_same_answers(&self) -> usize {
        self.0.iter().fold(0, |acc, GroupAnswers(group_answers)| {
            let group_same_answers =
                group_answers
                    .iter()
                    .fold(HashMap::with_capacity(26), |acc, PersonAnswers(s)| {
                        s.chars().into_iter().fold(acc, |mut acc, c| {
                            *acc.entry(c).or_insert(0) += 1;
                            acc
                        })
                    });
            let count_for_group = group_same_answers
                .iter()
                .filter(|(_, kount)| **kount == group_answers.len())
                .count();
            acc + count_for_group
        })
    }
}

fn parse(s: &str) -> GroupsAnswers {
    let split_by_newline = s.trim().split("\n\n"); // ugh.... really need to figure out how to do this with just combine...

    GroupsAnswers(
        split_by_newline
            .filter_map(|group| {
                let person_answers_parser = many::<String, _, _>(letter()).map(PersonAnswers);
                let mut group_people_answers_parser =
                    sep_by1(person_answers_parser, newline()).map(GroupAnswers);
                group_people_answers_parser
                    .easy_parse(group)
                    .ok()
                    .map(|r| r.0)
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
        let r = parse(input);
        let expected = GroupsAnswers(
            vec![
                vec!["abc"],
                vec!["a", "b", "c"],
                vec!["ab", "ac"],
                vec!["a", "a", "a", "a"],
                vec!["b"],
            ]
            .iter()
            .map(|v| GroupAnswers(v.iter().map(|s| PersonAnswers(s.to_string())).collect()))
            .collect(),
        );
        assert_eq!(expected, r);
    }

    #[test]
    fn sum_of_group_distinct_answers_test() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
        let r = parse(input);
        assert_eq!(11, r.sum_of_group_distinct_answers())
    }
    #[test]
    fn sum_of_group_same_answers_test() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b
";
        let r = parse(input);
        assert_eq!(6, r.sum_of_group_same_answers())
    }
}
