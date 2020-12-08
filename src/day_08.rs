use anyhow::Result;
use combine::easy;
use combine::parser::char::*;
use combine::*;

use combine::lib::collections::HashSet;
use std::num::ParseIntError;
use std::result::Result as StdResult;

const INPUT: &str = include_str!("../data/day_08_input");

pub fn run() -> Result<()> {
    println!("*** Day 8: Handheld Halting");
    println!("Input: {}", INPUT);
    let program = parse(INPUT)?;
    let mut interpreter = Interpreter::new(&program);

    interpreter.run_until_repeat();

    println!("Solution 1: {:?}", interpreter.accumulator);

    Ok(())
}

struct Interpreter<'a> {
    next_index: usize,
    visited_indices: HashSet<usize>,
    accumulator: isize,
    program: &'a Program,
}

impl<'a> Interpreter<'a> {
    fn new(program: &Program) -> Interpreter {
        Interpreter {
            next_index: 0,
            visited_indices: HashSet::new(),
            accumulator: 0,
            program,
        }
    }

    fn run_until_repeat(&mut self) {
        while let Some(instruction) = self.program.0.get(self.next_index) {
            if self.visited_indices.contains(&self.next_index) {
                break;
            }
            let current_index = self.next_index;
            match instruction {
                Instruction::Noop => self.next_index += 1,
                Instruction::Acc(by) => {
                    self.accumulator += by;
                    self.next_index += 1;
                }
                Instruction::Jump(jump) => {
                    let idx_as_i = self.next_index as isize;
                    self.next_index = (idx_as_i + jump) as usize;
                }
            }
            self.visited_indices.insert(current_index);
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Noop,
    Acc(isize),
    Jump(isize),
}

#[derive(Debug, PartialEq, Eq)]
struct Program(Vec<Instruction>);

fn i_number_parser<Input>() -> impl Parser<Input, Output = isize>
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
    char('+')
        .or(char('-'))
        .and(many::<String, _, _>(digit()))
        .and_then(|(sign, num)| {
            let combined = format!("{}{}", sign, num);
            combined.parse()
        })
}

fn parse(s: &str) -> StdResult<Program, easy::ParseError<&str>> {
    let single_instruction_parser = attempt(
        string("acc")
            .skip(space())
            .and(i_number_parser())
            .map(|(_, num)| Instruction::Acc(num))
            .or(attempt(
                string("jmp")
                    .skip(space())
                    .and(i_number_parser())
                    .map(|(_, num)| Instruction::Jump(num)),
            ))
            .or(attempt(
                string("nop")
                    .skip(space())
                    .and(i_number_parser())
                    .map(|(_, _)| Instruction::Noop),
            )),
    );
    let mut parser = many(single_instruction_parser.skip(spaces())).map(Program);
    let (r, _) = parser.easy_parse(s)?;
    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::Instruction::*;
    use super::*;

    #[test]
    fn parse_test() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
";
        let expected = Program(vec![
            Noop,
            Acc(1),
            Jump(4),
            Acc(3),
            Jump(-3),
            Acc(-99),
            Acc(1),
            Jump(-4),
            Acc(6),
        ]);
        let r = parse(input).unwrap();
        assert_eq!(expected, r);
    }

    #[test]
    fn interpreter_run_test() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
";
        let program = parse(input).unwrap();
        let mut interpreter = Interpreter::new(&program);
        interpreter.run_until_repeat();

        assert_eq!(5, interpreter.accumulator);
    }
}
