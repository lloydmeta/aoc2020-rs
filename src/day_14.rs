use anyhow::Result;
use combine::easy;
use combine::lib::collections::HashMap;
use combine::parser::char::*;
use combine::*;
use itertools::Itertools;
use std::num::ParseIntError;
use std::result::Result as StdResult;

const INPUT: &str = include_str!("../data/day_14_input");

pub fn run() -> Result<()> {
    println!("*** Day 14: Docking Data ***");
    println!("Input: {}", INPUT);
    let groups = parse(INPUT)?;
    let simulation_1 = Simulation::new(groups.clone());

    println!("Solution 1: {:?}", simulation_1.solution_1());

    let simulation_2 = Simulation::new(groups);
    println!("Solution 2: {:?}", simulation_2.solution_2());
    Ok(())
}

#[derive(Debug, Eq, PartialEq)]
struct Simulation {
    memory_space: HashMap<usize, u64>,
    groups: Vec<Group>,
}

impl Simulation {
    fn new(groups: Vec<Group>) -> Simulation {
        Simulation {
            memory_space: HashMap::new(),
            groups,
        }
    }

    fn solution_1(mut self) -> u64 {
        for group in self.groups.iter() {
            for mem_set in group.mem_sets.iter() {
                let mem_set_masked_value = group.mask.apply(mem_set.value);
                self.memory_space.insert(mem_set.idx, mem_set_masked_value);
            }
        }
        self.memory_space.values().sum()
    }

    fn solution_2(mut self) -> u64 {
        for group in self.groups.iter() {
            for mem_set in group.mem_sets.iter() {
                let mem_addresses = group.mask.address_decode(mem_set.idx);
                for mem_address in mem_addresses {
                    self.memory_space.insert(mem_address, mem_set.value);
                }
            }
        }
        self.memory_space.values().sum()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Mask {
    or_bitmask: u64,
    and_bitmask: u64,
    part_2_initial_or_bitmask: u64,
    part_2_xor_masks: Vec<u64>,
}

impl Mask {
    fn from_str(s: &str) -> StdResult<Mask, ParseIntError> {
        let (
            mut or_bitmask_vec_36_bits,
            mut and_bitmask_str_36_bits,
            mut part_2_initial_or_mask_bits,
            part_2_xor_masks,
        ) = s.chars().enumerate().fold(
            (vec!['0'; 36], vec!['1'; 36], vec!['0'; 36], vec![]),
            |(
                mut or_bitmask_str,
                mut and_bitmask_str,
                mut part_2_initial_or_mask,
                mut part_2_xor_masks,
            ),
             (idx, next_char)| {
                match next_char {
                    '1' => {
                        or_bitmask_str[idx] = '1';
                        part_2_initial_or_mask[idx] = '1';
                    }
                    '0' => and_bitmask_str[idx] = '0',
                    _ /* assume x only due to implementation */ => {
                        part_2_initial_or_mask[idx] = '1';
                        part_2_xor_masks.push(2u64.pow((s.len() - 1 - idx) as u32));
                    }
                }
                (
                    or_bitmask_str,
                    and_bitmask_str,
                    part_2_initial_or_mask,
                    part_2_xor_masks,
                )
            },
        );
        let or_bitmask_string: String = {
            let mut full_string = vec!['0'; 28];
            full_string.append(&mut or_bitmask_vec_36_bits);
            full_string.iter().collect()
        };
        let part_2_initial_or_mask_string: String = {
            let mut full_string = vec!['0'; 28];
            full_string.append(&mut part_2_initial_or_mask_bits);
            full_string.iter().collect()
        };
        let and_bitmask_string: String = {
            let mut full_string = vec!['1'; 28];
            full_string.append(&mut and_bitmask_str_36_bits);
            full_string.iter().collect()
        };

        let or_bitmask = u64::from_str_radix(&or_bitmask_string, 2);
        let and_bitmask = u64::from_str_radix(&and_bitmask_string, 2);
        let part_2_initial_or_bitmask = u64::from_str_radix(&part_2_initial_or_mask_string, 2);
        or_bitmask.and_then(|or_bitmask| {
            and_bitmask.and_then(|and_bitmask| {
                part_2_initial_or_bitmask.map(|part_2_initial_or_bitmask| Mask {
                    or_bitmask,
                    and_bitmask,
                    part_2_initial_or_bitmask,
                    part_2_xor_masks,
                })
            })
        })
    }

    fn apply(&self, i: u64) -> u64 {
        (i | self.or_bitmask) & self.and_bitmask
    }

    fn address_decode(&self, address: usize) -> Vec<usize> {
        let initial_ored_value = address as u64 | self.part_2_initial_or_bitmask;
        let xor_masks = (1..self.part_2_xor_masks.len() + 1)
            .into_iter()
            .flat_map(|xor_combo_length| {
                let xor_masks_at_length: Vec<_> = self
                    .part_2_xor_masks
                    .iter()
                    .combinations(xor_combo_length)
                    .map(|combination| combination.iter().fold(0, |acc, next| acc ^ **next))
                    .collect();
                xor_masks_at_length
            })
            .unique();
        let mut xored_combinations: Vec<_> = xor_masks
            .map(|or_mask| (initial_ored_value ^ or_mask) as usize)
            .collect();
        xored_combinations.push(initial_ored_value as usize);
        xored_combinations
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct MemSet {
    idx: usize,
    value: u64,
}

#[derive(Debug, Eq, PartialEq, Clone)]
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
        .and_then(|s| Mask::from_str(&s));

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
    fn mask_test() {
        let m = Mask::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").unwrap();
        let r1 = m.apply(11);
        assert_eq!(73, r1);
        let r2 = m.apply(101);
        assert_eq!(101, r2);
        let r3 = m.apply(0);
        assert_eq!(64, r3);
    }

    #[test]
    fn mask_address_decode_1_test() {
        let m = Mask::from_str("000000000000000000000000000000X1001X").unwrap();
        let mut r = m.address_decode(42);
        r.sort();
        assert_eq!(4, r.len());
        assert_eq!(vec![26, 27, 58, 59], r);
    }

    #[test]
    fn mask_address_decode_2_test() {
        let m = Mask::from_str("00000000000000000000000000000000X0XX").unwrap();
        let mut r = m.address_decode(26);
        r.sort();
        assert_eq!(8, r.len());
        assert_eq!(vec![16, 17, 18, 19, 24, 25, 26, 27], r);
    }

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
                    part_2_initial_or_bitmask: u64::from_str_radix(
                        "0000000000000000000000000000111111111111111111111111111111111101",
                        2,
                    )
                    .unwrap(),
                    part_2_xor_masks: vec![
                        34359738368,
                        17179869184,
                        8589934592,
                        4294967296,
                        2147483648,
                        1073741824,
                        536870912,
                        268435456,
                        134217728,
                        67108864,
                        33554432,
                        16777216,
                        8388608,
                        4194304,
                        2097152,
                        1048576,
                        524288,
                        262144,
                        131072,
                        65536,
                        32768,
                        16384,
                        8192,
                        4096,
                        2048,
                        1024,
                        512,
                        256,
                        128,
                        32,
                        16,
                        8,
                        4,
                        1,
                    ],
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
                    // XXXXXXXXXXXXXXXXXXXXXXXXXXXXX0XXXX1X
                    part_2_initial_or_bitmask: u64::from_str_radix(
                        "0000000000000000000000000000111111111111111111111111111110111111",
                        2,
                    )
                    .unwrap(),
                    part_2_xor_masks: vec![
                        34359738368,
                        17179869184,
                        8589934592,
                        4294967296,
                        2147483648,
                        1073741824,
                        536870912,
                        268435456,
                        134217728,
                        67108864,
                        33554432,
                        16777216,
                        8388608,
                        4194304,
                        2097152,
                        1048576,
                        524288,
                        262144,
                        131072,
                        65536,
                        32768,
                        16384,
                        8192,
                        4096,
                        2048,
                        1024,
                        512,
                        256,
                        128,
                        32,
                        16,
                        8,
                        4,
                        1,
                    ],
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

    #[test]
    fn solution_1_test() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
";
        let groups = parse(input).unwrap();
        let simulation = Simulation::new(groups);
        assert_eq!(165, simulation.solution_1());
    }
}
