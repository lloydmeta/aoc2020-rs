use crate::day_19::Rule::Letter;
use combine::easy;
use combine::parser::char::*;
use combine::*;
use std::num::ParseIntError;
use std::result::Result as StdResult;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Rule {
    Letter(char),
    AlternativeReferenceSequences(Vec<Vec<usize>>),
}

#[derive(Debug, PartialEq, Eq)]
struct Rules(Vec<Rule>);

fn parse(s: &str) -> StdResult<Rules, easy::ParseError<&str>> {
    let idx_parser = number().skip(char(':')).map(|idx| {
        println!("parsed index [{}]", idx);
        idx
    });

    let rule_parser = idx_parser
        .skip(spaces())
        .and(letter_rule().or(alt_sequence_ref_parser()));

    let mut rules_parser =
        many(rule_parser.skip(spaces())).map(|idx_with_rules: Vec<(usize, Rule)>| {
            let mut vec = vec![Rule::Letter('_'); idx_with_rules.len()];

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
    char('"')
        .with(letter())
        .skip(char('"'))
        .map(|l| Rule::Letter(l))
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
    fn day_19_single_parse_test() {
        let input = r#"0: 1 2
1: "a"
2: 1 3 | 3 1
3: "b"
"#;
        let r = parse(input).unwrap();

        let expected = Rules(vec![
            Rule::AlternativeReferenceSequences(vec![vec![1, 2]]),
            Rule::Letter('a'),
            Rule::AlternativeReferenceSequences(vec![vec![1, 3], vec![3, 1]]),
            Rule::Letter('b'),
        ]);

        assert_eq!(expected, r)
    }
}
