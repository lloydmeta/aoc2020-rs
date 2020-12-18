use anyhow::Result;

use combine::easy;
use combine::parser::char::*;
use combine::*;
use std::num::ParseIntError;
use std::result::Result as StdResult;

const INPUT: &str = include_str!("../data/day_18_input");

pub fn run() -> Result<()> {
    println!("*** Day 18: Operation Order ***");
    println!("Input: {}", INPUT);

    println!("Solution 1: {:?}", parse_1(INPUT)?);
    println!("Solution 2: {:?}", parse_2(INPUT)?);

    Ok(())
}

fn parse_1(s: &str) -> StdResult<usize, easy::ParseError<&str>> {
    let mut full_parser = many(expr().skip(spaces())).map(|nums: Vec<usize>| nums.iter().sum());
    let (r, _) = full_parser.easy_parse(s)?;
    Ok(r)
}

fn parse_2(s: &str) -> StdResult<usize, easy::ParseError<&str>> {
    let mut full_parser = many(expr_2().skip(spaces())).map(|nums: Vec<usize>| nums.iter().sum());
    let (r, _) = full_parser.easy_parse(s)?;
    Ok(r)
}

enum Op {
    Add,
    Multiply,
}

// This is a recursive parser....we need to use the macro to expose `expr` as a function, which
// actually calls `expr_`
fn expr_<Input>() -> impl Parser<Input, Output = usize>
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
    let factor =
        || number_parser().or(lex_char('(').with(expr().skip(spaces()).skip(lex_char(')'))));

    let op = lex_char('+')
        .map(|_| Op::Add)
        .or(lex_char('*').map(|_| Op::Multiply));
    factor()
        .and(many(op.and(factor())))
        .map(|(num, list): (usize, Vec<(Op, usize)>)| {
            list.iter().fold(num, |acc, next| match next {
                (Op::Add, next) => acc + next,
                (Op::Multiply, next) => acc * next,
            })
        })
}

parser! {
    fn expr[Input]()(Input) -> usize where [
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
    ]
    {
        expr_()
    }

}

fn expr_2_<Input>() -> impl Parser<Input, Output = usize>
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
    let factor =
        || number_parser().or(lex_char('(').with(expr_2().skip(spaces()).skip(lex_char(')'))));

    let term = || {
        factor()
            .and(many(lex_char('+').with(factor())))
            .map(|(num, list): (usize, Vec<usize>)| list.iter().fold(num, |acc, next| acc + next))
    };

    term()
        .and(many(lex_char('*').with(term())))
        .map(|(num, list): (usize, Vec<usize>)| list.iter().fold(num, |acc, next| acc * next))
}

parser! {
    fn expr_2[Input]()(Input) -> usize where [
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
    ]
    {
        expr_2_()
    }

}

fn expr_3_<Input>() -> impl Parser<Input, Output = usize>
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
    let factor =
        || number_parser().or(lex_char('(').with(expr_3().skip(spaces()).skip(lex_char(')'))));

    let term = || {
        factor()
            .and(many(lex_char('*').with(factor())))
            .map(|(num, list): (usize, Vec<usize>)| list.iter().fold(num, |acc, next| acc * next))
    };

    term()
        .and(many(lex_char('+').with(term())))
        .map(|(num, list): (usize, Vec<usize>)| list.iter().fold(num, |acc, next| acc + next))
}

parser! {
    fn expr_3[Input]()(Input) -> usize where [
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
    ]
    {
        expr_3_()
    }

}

fn lex_char<Input>(c: char) -> impl Parser<Input, Output = char>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    char(c).skip(spaces()).silent()
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
    many::<String, _, _>(digit())
        .and_then(|d| d.parse::<usize>())
        .skip(spaces())
}

#[cfg(test)]
mod parser_tests {

    use super::*;

    #[test]
    fn simple_parser_parse_test() {
        let input = "1 + 2 * 3 + 4 * 5 + 6";
        let mut parser = expr();
        let (r, _) = parser.easy_parse(input).unwrap();
        assert_eq!(71, r);
    }

    #[test]
    fn complex_parser_parse_test_1() {
        let input = "1 + (2 * 3) + (4 * (5 + 6))";
        let mut parser = expr();
        let (r, _) = parser.easy_parse(input).unwrap();
        assert_eq!(51, r);
    }

    #[test]
    fn complex_parser_parse_test_2() {
        let input = "2 * 3 + (4 * 5)";
        let mut parser = expr();
        let (r, _) = parser.easy_parse(input).unwrap();
        assert_eq!(26, r);
    }

    #[test]
    fn complex_parser_parse_test_3() {
        let input = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        let mut parser = expr();
        let (r, _) = parser.easy_parse(input).unwrap();
        assert_eq!(437, r);
    }

    #[test]
    fn complex_parser_parse_test_4() {
        let input = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        let mut parser = expr();
        let (r, _) = parser.easy_parse(input).unwrap();
        assert_eq!(12240, r);
    }

    #[test]
    fn complex_parser_parse_test_5() {
        let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let mut parser = expr();
        let (r, _) = parser.easy_parse(input).unwrap();
        assert_eq!(13632, r);
    }

    #[test]
    fn part_2_parser_parse_test() {
        let input = "1 + 2 * 3 + 4 * 5 + 6";
        let mut parser = expr_2();
        let (r, _) = parser.easy_parse(input).unwrap();
        assert_eq!(231, r);
    }

    #[test]
    fn complex_part_2_parser_parse_test_1() {
        let input = "1 + (2 * 3) + (4 * (5 + 6))";
        let mut parser = expr_2();
        let (r, _) = parser.easy_parse(input).unwrap();
        assert_eq!(51, r);
    }

    #[test]
    fn complex_part_2_parser_parse_test_2() {
        let input = "2 * 3 + (4 * 5)";
        let mut parser = expr_2();
        let (r, _) = parser.easy_parse(input).unwrap();
        assert_eq!(46, r);
    }

    #[test]
    fn complex_part_2_parser_parse_test_3() {
        let input = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        let mut parser = expr_2();
        let (r, _) = parser.easy_parse(input).unwrap();
        assert_eq!(1445, r);
    }

    #[test]
    fn complex_part_2_parser_parse_test_4() {
        let input = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        let mut parser = expr_2();
        let (r, _) = parser.easy_parse(input).unwrap();
        assert_eq!(669060, r);
    }

    #[test]
    fn complex_part_2_parser_parse_test_5() {
        let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let mut parser = expr_2();
        let (r, _) = parser.easy_parse(input).unwrap();
        assert_eq!(23340, r);
    }

    #[test]
    fn complex_part_3_parser_parse_test_1() {
        let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let mut parser = expr_3();
        let (r, _) = parser.easy_parse(input).unwrap();
        assert_eq!(((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2, r);
    }
}
