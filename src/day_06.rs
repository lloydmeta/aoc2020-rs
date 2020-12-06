use anyhow::Result;
use combine::easy::ParseError;
use combine::parser::char::*;
use combine::*;

use std::convert::TryInto;

use combine::lib::collections::HashSet;
use std::result::Result as StdResult;

#[derive(Debug, PartialEq, Eq)]
struct PersonAnswers(String);

#[derive(Debug, PartialEq, Eq)]
struct GroupAnswers(Vec<PersonAnswers>);

fn parse(s: &str) -> Vec<GroupAnswers> {
    let split_by_newline = s.split("\n\n"); // ugh.... really need to figure out how to do this with just combine...

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
        .collect()
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
        let expected: Vec<GroupAnswers> = vec![
            vec!["abc"],
            vec!["a", "b", "c"],
            vec!["ab", "ac"],
            vec!["a", "a", "a", "a"],
            vec!["b"],
        ]
        .iter()
        .map(|v| GroupAnswers(v.iter().map(|s| PersonAnswers(s.to_string())).collect()))
        .collect();
        assert_eq!(expected, r);
    }
}
