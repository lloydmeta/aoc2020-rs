use anyhow::Result;
use combine::easy;
use combine::parser::char::*;
use combine::*;
use num::integer::lcm;
use std::num::ParseIntError;
use std::result::Result as StdResult;

const INPUT: &str = include_str!("../data/day_14_input");

#[derive(Debug, Eq, PartialEq)]
struct Mask {
    or_bitmask: u64,
    and_bitmask: u64,
}

#[derive(Debug, Eq, PartialEq)]
struct MemSet {
    idx: usize,
    value: u64,
}

#[derive(Debug, Eq, PartialEq)]
struct Group {
    mask: Mask,

    mem_sets: Vec<MemSet>,
}

fn parse(s: &str) -> StdResult<Vec<Group>, easy::ParseError<&str>> {
    let mask_parser = attempt(string("mask"))
        .with(spaces())
        .with(char('='))
        .with(spaces())
        .with(many::<String, _, _>(one_of("X01".chars())))
        .and_then(|string| {
            let (mut or_bitmask_vec_36_bits, mut and_bitmask_str_36_bits) =
                string.chars().enumerate().fold(
                    (vec!['0'; 36], vec!['1'; 36]),
                    |(mut or_bitmask_str, mut and_bitmask_str), (idx, next_char)| {
                        match next_char {
                            '1' => or_bitmask_str[idx] = '1',
                            '0' => and_bitmask_str[idx] = '0',
                            _ => (),
                        }
                        (or_bitmask_str, and_bitmask_str)
                    },
                );
            let or_bitmask_string: String = {
                let mut full_string = vec!['0'; 28];
                full_string.append(&mut or_bitmask_vec_36_bits);
                full_string.iter().collect()
            };
            let and_bitmask_string: String = {
                let mut full_string = vec!['1'; 28];
                full_string.append(&mut and_bitmask_str_36_bits);
                full_string.iter().collect()
            };

            println!("or_bitmask_string {}", or_bitmask_string);
            println!("and_bitmask_string {}", and_bitmask_string);

            let or_bitmask = u64::from_str_radix(&or_bitmask_string, 2);
            let and_bitmask = u64::from_str_radix(&and_bitmask_string, 2);
            or_bitmask.and_then(|or_bitmask| {
                and_bitmask.map(|and_bitmask| Mask {
                    or_bitmask,
                    and_bitmask,
                })
            })
        });

    let mem_set_parser = attempt(string("mem["))
        .with(idx_parser())
        .skip(char(']').with(spaces()).with(char('=')).with(spaces()))
        .and(value_parser())
        .map(|(idx, value)| MemSet { idx, value });
    let group_parser = mask_parser
        .skip(newline())
        .and(many(mem_set_parser.skip(spaces())))
        .map(|(mask, mem_sets)| Group { mask, mem_sets });

    let mut full_parser = many(group_parser.skip(spaces()));
    let (r, _) = full_parser.easy_parse(s)?;
    Ok(r)
}

fn idx_parser<Input>() -> impl Parser<Input, Output = usize>
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

fn value_parser<Input>() -> impl Parser<Input, Output = u64>
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
    many::<String, _, _>(digit()).and_then(|d| d.parse::<u64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX0XXXX1X
mem[18] = 111
mem[17] = 1101
mem[18] = 10
";
        let r = parse(input).unwrap();
        assert_eq!(2, r.len()); // sanity check
        let expected = vec![
            Group {
                mask: Mask {
                    or_bitmask: u64::from_str_radix(
                        "0000000000000000000000000000000000000000000000000000000001000000",
                        2,
                    )
                    .unwrap(),
                    and_bitmask: u64::from_str_radix(
                        "1111111111111111111111111111111111111111111111111111111111111101",
                        2,
                    )
                    .unwrap(),
                },
                mem_sets: vec![
                    MemSet { idx: 8, value: 11 },
                    MemSet { idx: 7, value: 101 },
                    MemSet { idx: 8, value: 0 },
                ],
            },
            Group {
                mask: Mask {
                    or_bitmask: u64::from_str_radix(
                        "00000000000000000000000000000000000000000000000000000000000000010",
                        2,
                    )
                    .unwrap(),
                    and_bitmask: u64::from_str_radix(
                        "1111111111111111111111111111111111111111111111111111111110111111",
                        2,
                    )
                    .unwrap(),
                },
                mem_sets: vec![
                    MemSet {
                        idx: 18,
                        value: 111,
                    },
                    MemSet {
                        idx: 17,
                        value: 1101,
                    },
                    MemSet { idx: 18, value: 10 },
                ],
            },
        ];
        assert_eq!(expected, r);

        let actual_data = parse(INPUT).unwrap();
        assert!(!actual_data.is_empty())
    }
}
