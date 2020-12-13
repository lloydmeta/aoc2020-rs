use anyhow::Result;
use combine::easy;
use combine::parser::char::*;
use combine::*;
use std::num::ParseIntError;
use std::result::Result as StdResult;

#[derive(Debug, Eq, PartialEq)]
struct Notes {
    earliest_departure_minute: usize,
    buses: Vec<Bus>,
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
}
