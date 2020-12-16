use anyhow::Result;
use combine::easy;
use combine::lib::collections::{HashMap, HashSet};
use combine::parser::char::*;
use combine::*;
use std::collections::hash_map::Entry;
use std::num::ParseIntError;
use std::ops::RangeInclusive;
use std::result::Result as StdResult;

const INPUT: &str = include_str!("../data/day_16_input");

pub fn run() -> Result<()> {
    println!("*** Day 16: Ticket Translation ***");
    println!("Input: {}", INPUT);
    let data = parse(INPUT)?;

    println!("Solution 1: {:?}", data.solution_1());
    println!("Solution 2: {:?}", data.solution_2());

    Ok(())
}

#[derive(Debug, Eq, PartialEq)]
struct Rule {
    name: String,
    range_1: RangeInclusive<usize>,
    range_2: RangeInclusive<usize>,
}

impl Rule {
    fn is_valid(&self, check: usize) -> bool {
        self.range_1.contains(&check) || self.range_2.contains(&check)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Ticket(Vec<usize>);

#[derive(Debug, Eq, PartialEq)]
struct Data {
    rules: Vec<Rule>,
    your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl Data {
    fn solution_1(&self) -> usize {
        self.nearby_tickets
            .iter()
            .flat_map(|ticket| {
                ticket
                    .0
                    .iter()
                    .filter(|value| !self.rules.iter().any(|rule| rule.is_valid(**value)))
            })
            .sum()
    }

    fn solution_2(&self) -> usize {
        let mappings = self.field_idx_to_valid_rules();
        let mappings_that_start_with_departure = mappings
            .iter()
            .filter(|(_, (_, rule_name))| rule_name.starts_with("departure"));
        mappings_that_start_with_departure.fold(1, |acc, (field_idx, _)| {
            if let Some(value) = self.your_ticket.0.get(*field_idx) {
                acc * value
            } else {
                acc
            }
        })
    }

    fn field_idx_to_valid_rules(&self) -> HashMap<usize, (usize, &str)> {
        // prune invalid tickets

        let valid_tickets = self.nearby_tickets.iter().filter(|ticket| {
            let first_invalid_value = ticket
                .0
                .iter()
                .find(|value| !self.rules.iter().any(|rule| rule.is_valid(**value)));
            first_invalid_value.is_none()
        });

        let mut value_idx_to_possible_rule_names =
            valid_tickets.fold(HashMap::new(), |mut acc, ticket| {
                for (field_idx, value) in ticket.0.iter().enumerate() {
                    let valid_rule_names = self
                        .rules
                        .iter()
                        .enumerate()
                        .filter_map(|(rule_idx, rule)| {
                            if rule.is_valid(*value) {
                                Some((rule_idx, rule.name.as_ref()))
                            } else {
                                None
                            }
                        })
                        .collect();
                    match acc.entry(field_idx) {
                        Entry::Occupied(existing) => {
                            let existing_entry: &mut HashSet<(usize, &str)> = existing.into_mut();
                            *existing_entry = existing_entry
                                .intersection(&valid_rule_names)
                                .copied()
                                .collect();
                        }
                        Entry::Vacant(v) => {
                            v.insert(valid_rule_names);
                        }
                    }
                }
                acc
            });

        while value_idx_to_possible_rule_names
            .iter()
            .any(|(_, v)| v.len() > 1)
        {
            let field_idx_to_single_rule_idx: HashMap<_, _> = value_idx_to_possible_rule_names
                .iter()
                .filter_map(|(k, v)| {
                    if v.len() == 1 {
                        v.iter().last().copied().map(|p| (*k, p.0))
                    } else {
                        None
                    }
                })
                .collect();
            let rule_idx_bound_to_single_field_idx: HashSet<usize> =
                field_idx_to_single_rule_idx.values().copied().collect();
            for (_, rule_idx_and_name) in value_idx_to_possible_rule_names
                .iter_mut()
                .filter(|(field_idx, _)| !field_idx_to_single_rule_idx.contains_key(field_idx))
            {
                rule_idx_and_name
                    .retain(|(rule_idx, _)| !rule_idx_bound_to_single_field_idx.contains(rule_idx))
            }
        }

        value_idx_to_possible_rule_names
            .into_iter()
            .filter_map(|(k, v)| {
                if let Some(v) = v.into_iter().last() {
                    Some((k, v))
                } else {
                    None
                }
            })
            .collect()
    }
}

fn parse(s: &str) -> StdResult<Data, easy::ParseError<&str>> {
    // ugh

    let split: Vec<_> = s.trim().split("\n\n").collect();

    let (rules, _) = rules_parser().easy_parse(split[0])?;

    let mut your_ticket_parser = string("your ticket:")
        .skip(newline())
        .with(sep_by1(number_parser(), char(',')).map(Ticket));

    let (your_ticket, _) = your_ticket_parser.easy_parse(split[1])?;

    let mut nearby_tickets_parser = string("nearby tickets:").skip(newline()).with(sep_by1(
        sep_by1(number_parser(), char(',')).map(Ticket),
        newline(),
    ));

    let (nearby_tickets, _) = nearby_tickets_parser.easy_parse(split[2])?;
    Ok(Data {
        rules,
        your_ticket,
        nearby_tickets,
    })
}

fn rules_parser<Input>() -> impl Parser<Input, Output = Vec<Rule>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    <<Input as StreamOnce>::Error as combine::ParseError<
        char,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >>::StreamError: From<ParseIntError>,
    <<Input as StreamOnce>::Error as combine::ParseError<
        char,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >>::StreamError: From<ParseIntError>,
    <Input as combine::StreamOnce>::Error: combine::ParseError<
        char,
        <Input as combine::StreamOnce>::Range,
        <Input as combine::StreamOnce>::Position,
    >,
{
    let rule_parser = many::<String, _, _>(letter().or(space()))
        .skip(char(':'))
        .skip(spaces())
        .and(number_parser())
        .skip(char('-'))
        .and(number_parser())
        .skip(spaces())
        .skip(string("or"))
        .skip(spaces())
        .and(number_parser())
        .skip(char('-'))
        .and(number_parser())
        .map(
            |((((name, range_1_start), range_1_end), range_2_start), range_2_end)| Rule {
                name,
                range_1: range_1_start..=range_1_end,
                range_2: range_2_start..=range_2_end,
            },
        );
    sep_by1(rule_parser, (newline(), not_followed_by(newline())))
}

fn number_parser<Input>() -> impl Parser<Input, Output = usize>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    <<Input as StreamOnce>::Error as combine::ParseError<
        char,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >>::StreamError: From<ParseIntError>,
    <<Input as StreamOnce>::Error as combine::ParseError<
        char,
        <Input as StreamOnce>::Range,
        <Input as StreamOnce>::Position,
    >>::StreamError: From<ParseIntError>,
    <Input as combine::StreamOnce>::Error: combine::ParseError<
        char,
        <Input as combine::StreamOnce>::Range,
        <Input as combine::StreamOnce>::Position,
    >,
{
    many::<String, _, _>(digit()).and_then(|d| d.parse::<usize>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rules_parse_test() {
        let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50";
        let mut parser = rules_parser();
        let (r, _) = parser.easy_parse(input).unwrap();
        assert_eq!(3, r.len())
    }

    #[test]
    fn your_ticket_parse_test() {
        let input = "your ticket:
7,1,14";

        let mut parser = string("your ticket:")
            .skip(newline())
            .with(sep_by1(number_parser(), char(',')).map(Ticket));

        let (r, _) = parser.easy_parse(input).unwrap();
        assert_eq!(3, r.0.len())
    }

    #[test]
    fn parse_test() {
        let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
";
        let r = parse(input).unwrap();
        let expected = Data {
            rules: vec![
                Rule {
                    name: "class".to_string(),
                    range_1: 1..=3,
                    range_2: 5..=7,
                },
                Rule {
                    name: "row".to_string(),
                    range_1: 6..=11,
                    range_2: 33..=44,
                },
                Rule {
                    name: "seat".to_string(),
                    range_1: 13..=40,
                    range_2: 45..=50,
                },
            ],

            your_ticket: Ticket(vec![7, 1, 14]),
            nearby_tickets: vec![
                Ticket(vec![7, 3, 47]),
                Ticket(vec![40, 4, 50]),
                Ticket(vec![55, 2, 20]),
                Ticket(vec![38, 6, 12]),
            ],
        };
        assert_eq!(expected, r)
    }

    #[test]
    fn solution_1_test() {
        let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
";
        let r = parse(input).unwrap();
        assert_eq!(71, r.solution_1())
    }

    #[test]
    fn field_rules_test() {
        let input = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9
";
        let data = parse(input).unwrap();
        let mappings = data.field_idx_to_valid_rules();

        assert_eq!("row", mappings.get(&0).unwrap().1);
        assert_eq!("class", mappings.get(&1).unwrap().1);
        assert_eq!("seat", mappings.get(&2).unwrap().1);
    }
}
