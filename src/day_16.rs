use anyhow::Result;
use combine::easy;
use combine::lib::collections::HashMap;
use combine::parser::char::*;
use combine::*;
use itertools::Itertools;
use std::num::ParseIntError;
use std::ops::RangeInclusive;
use std::result::Result as StdResult;

#[derive(Debug, Eq, PartialEq)]
struct Rule {
    name: String,
    range_1: RangeInclusive<usize>,
    range_2: RangeInclusive<usize>,
}

#[derive(Debug, Eq, PartialEq)]
struct Ticket(Vec<usize>);

#[derive(Debug, Eq, PartialEq)]
struct Data {
    rules: Vec<Rule>,
    your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

fn parse(s: &str) -> StdResult<Data, easy::ParseError<&str>> {
    // ugh

    let split: Vec<_> = s.trim().split("\n\n").collect();
    println!("split {:?}", split);

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
}
