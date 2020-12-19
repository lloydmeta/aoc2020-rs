use anyhow::Result;

use combine::easy;
use combine::parser::char::*;
use combine::*;
use std::num::ParseIntError;
use std::result::Result as StdResult;

const INPUT: &str = include_str!("../data/day_19_input");
const INPUT_2: &str = include_str!("../data/day_19_input_part_2");

pub fn run() -> Result<()> {
    println!("*** Day 19: Monster Messages ***");
    println!("Input: {}", INPUT);

    let rules_with_messages = parse(INPUT)?;

    println!(
        "Solution 1: {:?}",
        rules_with_messages.count_messages_matching_rule_idx(0)
    );

    let rules_with_messages_2 = parse(INPUT_2)?;
    println!(
        "Solution 2: {:?}",
        rules_with_messages_2.count_messages_matching_rule_idx(0)
    );
    Ok(())
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Rule {
    Letter(char),
    // usize represents the "index" of the rule being referenced
    AlternativeReferenceSequences(Vec<Vec<usize>>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct RulesWithMessages {
    rules: Rules,
    messages: Vec<String>,
}

impl RulesWithMessages {
    fn count_messages_matching_rule_idx(&self, rule_idx: usize) -> usize {
        self.messages
            .iter()
            .filter(|msg| self.check_matches_rule(msg.as_str(), rule_idx))
            .count()
    }

    fn check_matches_rule(&self, s: &str, rule_idx: usize) -> bool {
        self.recursively_check_rules(s, &mut vec![rule_idx])
    }

    fn recursively_check_rules(
        &self,
        str_to_check: &str,
        mut rules_idx_to_check: &mut Vec<usize>,
    ) -> bool {
        if str_to_check.is_empty() && rules_idx_to_check.is_empty() {
            true
        } else {
            rules_idx_to_check
                .pop()
                .and_then(|rule_idx| {
                    self.rules.0.get(rule_idx).map(|rule| match rule {
                        Rule::Letter(char_to_check) => {
                            self.check_char(*char_to_check, str_to_check, &mut rules_idx_to_check)
                        }
                        Rule::AlternativeReferenceSequences(alternative_rule_idx_seqs) => {
                            alternative_rule_idx_seqs
                                .iter()
                                .any(|alternative_rule_idx_seq| {
                                    // Clone ensure that each alternative sequence is checked independently
                                    let mut rule_idx_to_check_for_alternative_seq =
                                        rules_idx_to_check.clone();
                                    self.check_rule_seq(
                                        alternative_rule_idx_seq,
                                        str_to_check,
                                        &mut rule_idx_to_check_for_alternative_seq,
                                    )
                                })
                        }
                    })
                })
                .unwrap_or(false)
        }
    }

    fn check_rule_seq(
        &self,
        rule_idx_seq: &[usize],
        s: &str,
        rules_idx_to_check: &mut Vec<usize>,
    ) -> bool {
        // Pump the sequence into rules to check and check it
        rule_idx_seq
            .iter()
            .rev()
            .for_each(|rule_idx| rules_idx_to_check.push(*rule_idx));
        self.recursively_check_rules(s, rules_idx_to_check)
    }

    fn check_char(
        &self,
        char_to_check: char,
        s: &str,
        rules_idx_to_check: &mut Vec<usize>,
    ) -> bool {
        match s.chars().next() {
            Some(next_char) if char_to_check == next_char => {
                let remaining_str: String = s.chars().skip(1).collect();
                self.recursively_check_rules(&remaining_str, rules_idx_to_check)
            }
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Rules(Vec<Rule>);

fn parse(s: &str) -> StdResult<RulesWithMessages, easy::ParseError<&str>> {
    let rules_with_messages: Vec<_> = s.trim().split("\n\n").collect();

    let rules = parse_rules(rules_with_messages[0])?;

    let messages = rules_with_messages[1]
        .split('\n')
        .map(|s| s.to_string())
        .collect();

    Ok(RulesWithMessages { rules, messages })
}

fn parse_rules(s: &str) -> StdResult<Rules, easy::ParseError<&str>> {
    let idx_parser = number().skip(char(':')).map(|idx| {
        // println!("parsed index [{}]", idx);
        idx
    });

    let rule_parser = idx_parser
        .skip(spaces())
        .and(letter_rule().or(alt_sequence_ref_parser()));

    let mut rules_parser =
        many(rule_parser.skip(spaces())).map(|idx_with_rules: Vec<(usize, Rule)>| {
            let length = idx_with_rules
                .iter()
                .map(|(idx, _)| *idx)
                .max()
                .unwrap_or(idx_with_rules.len());
            let mut vec = vec![Rule::Letter('_'); length + 1];

            for (idx, rule) in idx_with_rules.into_iter() {
                vec[idx] = rule;
            }
            Rules(vec)
        });

    let (r, _) = rules_parser.easy_parse(s)?;

    Ok(r)
}

fn alt_sequence_ref_parser<Input>() -> impl Parser<Input, Output = Rule>
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
    // let sequence_ref_parser = number().skip(char(' ')).and(number());
    // let sequence_ref_parser = sep_by1(number(), (not_followed_by(char('|')), char(' '))).map(|r| {
    //     println!("reference sequence [{:?}]", r);
    //     r
    // }); //skip(not_followed_by(char('|')));

    // ghetto
    let parser = many::<String, _, _>(digit().or(one_of(" |".chars()))).map(|s| {
        let alternate_sequences = s
            .trim()
            .split('|')
            .map(|section| {
                let v = section
                    .split(' ')
                    .filter_map(|s| s.parse::<usize>().ok())
                    .collect();
                v
            })
            .collect();
        Rule::AlternativeReferenceSequences(alternate_sequences)
    });

    parser
}

fn letter_rule<Input>() -> impl Parser<Input, Output = Rule>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    char('"').with(letter()).skip(char('"')).map(Rule::Letter)
}

fn number<Input>() -> impl Parser<Input, Output = usize>
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
    fn day_19_parse_rules_test() {
        let input = r#"0: 1 2
1: "a"
2: 1 3 | 3 1
3: "b"
"#;
        let r = parse_rules(input).unwrap();

        let expected = Rules(vec![
            Rule::AlternativeReferenceSequences(vec![vec![1, 2]]),
            Rule::Letter('a'),
            Rule::AlternativeReferenceSequences(vec![vec![1, 3], vec![3, 1]]),
            Rule::Letter('b'),
        ]);

        assert_eq!(expected, r)
    }

    #[test]
    fn day_19_parse_input_test() {
        let r = parse(INPUT).unwrap();
        assert!(!r.rules.0.is_empty());
    }

    #[test]
    fn day_19_rules_solution_1_test() {
        let input = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb
"#;
        let rules_with_messages = parse(input).unwrap();

        let r = rules_with_messages.count_messages_matching_rule_idx(0);
        assert_eq!(2, r)
    }

    #[test]
    fn day_19_rules_solution_2_test() {
        let input = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31 | 42 11 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42 | 42 8
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
"#;
        let rules_with_messages = parse(input).unwrap();

        let r = rules_with_messages.count_messages_matching_rule_idx(0);
        assert_eq!(12, r)
    }
}
