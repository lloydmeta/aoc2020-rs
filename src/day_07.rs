use anyhow::Result;
use combine::easy;
use combine::parser::char::*;
use combine::*;

use combine::lib::collections::HashSet;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::result::Result as StdResult;

const INPUT: &str = include_str!("../data/day_07_input");

pub fn run() -> Result<()> {
    println!("*** Day 7: Handy Haversacks ***");
    println!("Input: {}", INPUT);
    // println!("Input: {}", INPUT);
    let all_rules = parse(INPUT)?;

    let target_colour = BagColour("shiny gold".to_string());
    println!(
        "Solution 1: {:?}",
        find_bags_that_can_eventually_contain(&all_rules, &target_colour).len()
    );

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct BagColour(String);

fn find_bags_that_can_eventually_contain<'a>(
    total_rules: &'a HashMap<BagColour, HashMap<BagColour, usize>>,
    target_colour: &'a BagColour,
) -> HashSet<&'a BagColour> {
    let contains_target = total_rules
        .iter()
        .fold(HashSet::new(), |mut acc, (c, rules)| {
            if rules.get(target_colour).filter(|i| **i > 0).is_some() {
                acc.insert(c);
                acc
            } else {
                acc
            }
        });
    let containers_of_containers = contains_target
        .iter()
        .fold(HashSet::new(), |acc, container| {
            let contains_container = find_bags_that_can_eventually_contain(total_rules, *container);
            acc.union(&contains_container).copied().collect()
        });
    contains_target
        .union(&containers_of_containers)
        .copied()
        .collect()
}

fn parse(
    s: &str,
) -> StdResult<HashMap<BagColour, HashMap<BagColour, usize>>, easy::ParseError<&str>> {
    let mut parser = many(single_bag_colour_rules_parser().skip(spaces()));
    let (r, _) = parser.easy_parse(s)?;
    Ok(r)
}

fn single_bag_colour_rules_parser<Input>(
) -> impl Parser<Input, Output = (BagColour, HashMap<BagColour, usize>)>
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
    let rules_parser = {
        let rule_parser = number_parser()
            .skip(space())
            .and(bag_colour_string_parser().map(|s| {
                let no_suffix = s
                    .trim()
                    .strip_suffix(" bag")
                    .or_else(|| s.trim().strip_suffix(" bags"))
                    .unwrap_or(&s)
                    .to_owned();
                BagColour(no_suffix)
            }));
        sep_by(rule_parser, string(", "))
            .skip(char('.'))
            .map(|v: Vec<(usize, BagColour)>| {
                let mut h = HashMap::new();
                for (i, c) in v.into_iter() {
                    h.insert(c, i);
                }
                h
            })
    };

    let no_rules_parser = string("no other bags.").map(|_| HashMap::with_capacity(0));

    bag_colour_string_parser()
        .map(|s| {
            let maybe_without_suffix = s
                .trim()
                .strip_suffix(" bags contain")
                .or_else(|| s.trim().strip_suffix(" bags contain no other bags"));
            let without_suffix = maybe_without_suffix.unwrap_or(&s).to_owned();
            BagColour(without_suffix)
        })
        // .skip(string(" bags contain "))
        .and(rules_parser.or(no_rules_parser))
}

fn bag_colour_string_parser<Input>() -> impl Parser<Input, Output = String>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many::<String, _, _>(letter().or(space()))
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
    fn parse_single_with_rules_test() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let expected = {
            let mut h = HashMap::new();
            h.insert(BagColour("bright white".to_string()), 1);
            h.insert(BagColour("muted yellow".to_string()), 2);
            (BagColour("light red".to_string()), h)
        };
        let mut parser = single_bag_colour_rules_parser();
        let r = parser.easy_parse(input).unwrap().0;
        assert_eq!(expected, r);
    }

    #[test]
    fn parse_single_with_no_rules_test() {
        let input = "faded blue bags contain no other bags.";
        let expected = (BagColour("faded blue".to_string()), HashMap::new());
        let mut parser = single_bag_colour_rules_parser();
        let r = parser.easy_parse(input).unwrap().0;
        assert_eq!(expected, r);
    }

    #[test]
    fn parse_multiple_test() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";
        let mut expected = HashMap::new();
        expected.insert(BagColour("light red".to_string()), {
            let mut h = HashMap::new();
            h.insert(BagColour("bright white".to_string()), 1);
            h.insert(BagColour("muted yellow".to_string()), 2);
            h
        });
        expected.insert(BagColour("dark orange".to_string()), {
            let mut h = HashMap::new();
            h.insert(BagColour("bright white".to_string()), 3);
            h.insert(BagColour("muted yellow".to_string()), 4);
            h
        });
        expected.insert(BagColour("bright white".to_string()), {
            let mut h = HashMap::new();
            h.insert(BagColour("shiny gold".to_string()), 1);
            h
        });
        expected.insert(BagColour("muted yellow".to_string()), {
            let mut h = HashMap::new();
            h.insert(BagColour("shiny gold".to_string()), 2);
            h.insert(BagColour("faded blue".to_string()), 9);
            h
        });
        expected.insert(BagColour("shiny gold".to_string()), {
            let mut h = HashMap::new();
            h.insert(BagColour("dark olive".to_string()), 1);
            h.insert(BagColour("vibrant plum".to_string()), 2);
            h
        });
        expected.insert(BagColour("dark olive".to_string()), {
            let mut h = HashMap::new();
            h.insert(BagColour("faded blue".to_string()), 3);
            h.insert(BagColour("dotted black".to_string()), 4);
            h
        });

        expected.insert(BagColour("vibrant plum".to_string()), {
            let mut h = HashMap::new();
            h.insert(BagColour("faded blue".to_string()), 5);
            h.insert(BagColour("dotted black".to_string()), 6);
            h
        });
        expected.insert(BagColour("faded blue".to_string()), {
            let h = HashMap::new();
            h
        });
        expected.insert(BagColour("dotted black".to_string()), {
            let h = HashMap::new();
            h
        });

        let r = parse(input).unwrap();
        assert_eq!(expected, r);
    }

    #[test]
    fn find_bags_that_eventually_contain_test() {
        let mut rules = HashMap::new();
        rules.insert(BagColour("light red".to_string()), {
            let mut h = HashMap::new();
            h.insert(BagColour("bright white".to_string()), 1);
            h.insert(BagColour("muted yellow".to_string()), 2);
            h
        });
        rules.insert(BagColour("dark orange".to_string()), {
            let mut h = HashMap::new();
            h.insert(BagColour("bright white".to_string()), 3);
            h.insert(BagColour("muted yellow".to_string()), 4);
            h
        });
        rules.insert(BagColour("bright white".to_string()), {
            let mut h = HashMap::new();
            h.insert(BagColour("shiny gold".to_string()), 1);
            h
        });
        rules.insert(BagColour("muted yellow".to_string()), {
            let mut h = HashMap::new();
            h.insert(BagColour("shiny gold".to_string()), 2);
            h.insert(BagColour("faded blue".to_string()), 9);
            h
        });
        rules.insert(BagColour("shiny gold".to_string()), {
            let mut h = HashMap::new();
            h.insert(BagColour("dark olive".to_string()), 1);
            h.insert(BagColour("vibrant plum".to_string()), 2);
            h
        });
        rules.insert(BagColour("dark olive".to_string()), {
            let mut h = HashMap::new();
            h.insert(BagColour("faded blue".to_string()), 3);
            h.insert(BagColour("dotted black".to_string()), 4);
            h
        });

        rules.insert(BagColour("vibrant plum".to_string()), {
            let mut h = HashMap::new();
            h.insert(BagColour("faded blue".to_string()), 5);
            h.insert(BagColour("dotted black".to_string()), 6);
            h
        });
        rules.insert(BagColour("faded blue".to_string()), {
            let h = HashMap::new();
            h
        });
        rules.insert(BagColour("dotted black".to_string()), {
            let h = HashMap::new();
            h
        });
        let target_colour = BagColour("shiny gold".to_string());
        let bags_that_contain_shiny_gold =
            find_bags_that_can_eventually_contain(&rules, &target_colour);
        println!(
            "bags_that_contain_shiny_gold {:?}",
            bags_that_contain_shiny_gold
        );
        assert_eq!(4, bags_that_contain_shiny_gold.len())
    }
}
