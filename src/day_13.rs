use anyhow::Result;
use combine::easy;
use combine::parser::char::*;
use combine::*;
use num::integer::lcm;
use std::num::ParseIntError;
use std::result::Result as StdResult;

const INPUT: &str = include_str!("../data/day_13_input");

pub fn run() -> Result<()> {
    println!("*** Day 13: Shuttle Search ***");
    println!("Input: {}", INPUT);
    let notes = parse(INPUT)?;

    println!("Solution 1: {:?}", notes.solution_1());
    println!("Solution 2: {:?}", notes.solution_2());

    Ok(())
}

#[derive(Debug, Eq, PartialEq)]
struct Notes {
    earliest_departure_minute: u128,
    buses: Vec<Bus>,
}

impl Notes {
    fn bus_departures(&self) -> impl Iterator<Item = (u128, Vec<&Bus>)> {
        (0..).into_iter().filter_map(move |minute| {
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

    fn solution_1(&self) -> Option<u128> {
        let (time, buses) = self
            .bus_departures()
            .find(|(time, _)| *time >= self.earliest_departure_minute)?;
        let first_bus = buses.first()?;

        if let Bus::Id(id) = first_bus {
            Some(id * (time - self.earliest_departure_minute))
        } else {
            None
        }
    }

    fn solution_2(&self) -> Option<u128> {
        let bus_ids_with_time_offsets: Vec<_> = self
            .buses
            .iter()
            .enumerate()
            .filter_map(|(idx, bus)| {
                if let Bus::Id(id) = bus {
                    Some((id, idx as u128))
                } else {
                    None
                }
            })
            .collect();

        let (first_bus_id, _) = bus_ids_with_time_offsets.first()?;
        let (bus_0_departure, _) = bus_ids_with_time_offsets.iter().skip(1).try_fold(
            (0, **first_bus_id),
            |(current_first_bus_departure, current_bus_group_period),
             (next_bus_period, next_bus_offset)| {
                // The next departure time for the first bus that matches the period and offset of the next bus
                let next_bus_group_departure = (current_first_bus_departure..)
                    .step_by(current_bus_group_period as usize)
                    .find(|departure_time| {
                        (*departure_time + next_bus_offset) % **next_bus_period == 0
                    })?;

                // The combined period for all the buses we've visited, including this new, "next" one.
                // By incrementing with this "new" combined period, we ensure that the offsets
                // of the previous visited buses are taken into account

                // e.g. given (7, 13, x, x, 59)
                // 1. Between 7 (offset 0) and 13 (offset 1), the first offset is 77, (77/7 = 1, 78/13 = 6)
                //    New period is LCM of them, which is 91
                // 2. Next, find one that works ith 59, offset 4:
                //    [(77 + x *(91)) + 4]  % 59 = 0, where the first group is te next departure time,
                //     just keep incrementing x
                //    next departure time = 77 * 3 * 91  = 350
                //    check: 350 / 7 = 50, 351/13 = 27, 354/59 = 6
                let new_bus_group_period = lcm(current_bus_group_period, **next_bus_period);

                Some((next_bus_group_departure, new_bus_group_period))
            },
        )?;

        Some(bus_0_departure)
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Bus {
    Id(u128),
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

fn number_parser<Input>() -> impl Parser<Input, Output = u128>
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
    many::<String, _, _>(digit()).and_then(|d| d.parse::<u128>())
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
        assert_eq!(Some(295), notes.solution_1())
    }

    #[test]
    fn solution_2_test() {
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
        assert_eq!(Some(1068781), notes.solution_2())
    }

    #[test]
    fn solution_2_2_test() {
        let notes = Notes {
            earliest_departure_minute: 939,
            buses: vec![Bus::Id(17), Bus::NotInService, Bus::Id(13), Bus::Id(19)],
        };
        assert_eq!(Some(3417), notes.solution_2())
    }

    #[test]
    fn solution_2_3_test() {
        let notes = Notes {
            earliest_departure_minute: 939,
            buses: vec![Bus::Id(67), Bus::Id(7), Bus::Id(59), Bus::Id(61)],
        };
        assert_eq!(Some(754018), notes.solution_2())
    }

    #[test]
    fn solution_2_4_test() {
        let notes = Notes {
            earliest_departure_minute: 939,
            buses: vec![
                Bus::Id(67),
                Bus::NotInService,
                Bus::Id(7),
                Bus::Id(59),
                Bus::Id(61),
            ],
        };
        assert_eq!(Some(779210), notes.solution_2())
    }

    #[test]
    fn solution_2_5_test() {
        let notes = Notes {
            earliest_departure_minute: 939,
            buses: vec![
                Bus::Id(67),
                Bus::Id(7),
                Bus::NotInService,
                Bus::Id(59),
                Bus::Id(61),
            ],
        };
        assert_eq!(Some(1261476), notes.solution_2())
    }

    #[test]
    fn solution_2_6_test() {
        let notes = Notes {
            earliest_departure_minute: 939,
            buses: vec![Bus::Id(1789), Bus::Id(37), Bus::Id(47), Bus::Id(1889)],
        };
        assert_eq!(Some(1202161486), notes.solution_2())
    }
}
