use anyhow::Result;
use combine::easy;
use combine::parser::char::*;
use combine::*;
use std::num::ParseIntError;
use std::result::Result as StdResult;

const INPUT: &str = include_str!("../data/day_13_input");

pub fn run() -> Result<()> {
    println!("*** Day 13: Shuttle Search ***");
    println!("Input: {}", INPUT);
    let notes = parse(INPUT)?;

    println!("Solution 1: {:?}", notes.solution_1());

    Ok(())
}

#[derive(Debug, Eq, PartialEq)]
struct Notes {
    earliest_departure_minute: usize,
    buses: Vec<Bus>,
}

impl Notes {
    fn bus_departures(&self) -> impl Iterator<Item = (usize, Vec<&Bus>)> {
        (self.earliest_departure_minute..)
            .into_iter()
            .filter_map(move |minute| {
                let valid_buses: Vec<&Bus> = self
                    .buses
                    .iter()
                    .filter(|bus| {
                        if let Bus::Id(id) = bus {
                            minute % id == 0
                        } else {
                            false
                        }
                    })
                    .collect();
                if valid_buses.is_empty() {
                    None
                } else {
                    Some((minute, valid_buses))
                }
            })
    }

    fn solution_1(&self) -> Option<usize> {
        let earliest_departure_with_buses = self.bus_departures().next();
        earliest_departure_with_buses.and_then(|(time, buses)| {
            buses.first().and_then(|first_bus| {
                if let Bus::Id(id) = first_bus {
                    Some(id * (time - self.earliest_departure_minute))
                } else {
                    None
                }
            })
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Bus {
    Id(usize),
    NotInService,
}

fn parse(s: &str) -> StdResult<Notes, easy::ParseError<&str>> {
    let earliest_departure_minute_parser = number_parser();
    let bus_parser = choice!(
        number_parser().map(Bus::Id),
        char('x').map(|_| Bus::NotInService)
    );
    let buses_parser = sep_by(bus_parser, char(','));
    let mut parser = earliest_departure_minute_parser
        .skip(newline())
        .and(buses_parser)
        .map(|(earliest_departure_minute, buses)| Notes {
            earliest_departure_minute,
            buses,
        });
    let (r, _) = parser.easy_parse(s)?;
    Ok(r)
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
    fn parse_test() {
        let input = "939
7,13,x,x,59,x,31,19
";
        let r = parse(input).unwrap();
        let expected = Notes {
            earliest_departure_minute: 939,
            buses: vec![
                Bus::Id(7),
                Bus::Id(13),
                Bus::NotInService,
                Bus::NotInService,
                Bus::Id(59),
                Bus::NotInService,
                Bus::Id(31),
                Bus::Id(19),
            ],
        };
        assert_eq!(expected, r);
    }

    #[test]
    fn notes_bus_departures_test() {
        let notes = Notes {
            earliest_departure_minute: 939,
            buses: vec![
                Bus::Id(7),
                Bus::Id(13),
                Bus::NotInService,
                Bus::NotInService,
                Bus::Id(59),
                Bus::NotInService,
                Bus::Id(31),
                Bus::Id(19),
            ],
        };
        let (first_departure_time, first_departure_buses) = notes.bus_departures().next().unwrap();
        assert_eq!(944, first_departure_time);
        assert_eq!(vec![&Bus::Id(59)], first_departure_buses);
        assert_eq!(Some(295), notes.solution_1())
    }
}
