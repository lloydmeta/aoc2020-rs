use anyhow::Result;
use combine::easy;
use combine::parser::char::*;
use combine::*;
use std::num::ParseIntError;
use std::result::Result as StdResult;

use Action::*;
use Degrees::*;

const INPUT: &str = include_str!("../data/day_12_input");

pub fn run() -> Result<()> {
    println!("*** Day 12: Rain Risk ***");
    println!("Input: {}", INPUT);
    let actions = parse(INPUT)?;

    let mut simulation = Interpreter::new(actions.clone());
    simulation.run();
    let solution_1 = simulation.current_coords.manhattan_distance();
    println!("Solution 1: {:?}", solution_1);

    let mut simulation_2 = WaypointInterpreter::new(actions);
    simulation_2.run();
    let solution_2 = simulation_2.current_coords.manhattan_distance();
    println!("Solution 2: {:?}", solution_2);

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Degrees {
    _90,
    _180,
    _270,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RotateTo {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

static DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

impl Direction {
    fn rotate(&self, to: &RotateTo, rotate: &Degrees) -> Direction {
        let diff = match rotate {
            Degrees::_90 => 1,
            Degrees::_180 => 2,
            Degrees::_270 => 3,
        };
        let current_idx = match self {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        };

        let rotated_idx = match to {
            RotateTo::Left => -diff,
            RotateTo::Right => diff,
        };

        let new_idx: isize = current_idx + rotated_idx;
        let usize_idx = if new_idx < 0 {
            DIRECTIONS.len() - (new_idx.abs() as usize % DIRECTIONS.len())
        } else if new_idx >= DIRECTIONS.len() as isize {
            (new_idx % DIRECTIONS.len() as isize) as usize
        } else {
            new_idx as usize
        };
        DIRECTIONS[usize_idx]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Action {
    North(usize),
    South(usize),
    East(usize),
    West(usize),
    Rotate(RotateTo, Degrees),
    Forward(usize),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Coords {
    x: isize,
    y: isize,
}

impl Coords {
    fn manhattan_distance(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
}

struct Interpreter {
    current_direction: Direction,
    current_coords: Coords,
    instructions: Vec<Action>,
}

impl Interpreter {
    fn new(instructions: Vec<Action>) -> Interpreter {
        Interpreter {
            current_coords: Coords { x: 0, y: 0 },
            current_direction: Direction::East,
            instructions,
        }
    }

    fn run(&mut self) {
        for action in self.instructions.iter() {
            match action {
                North(by) => {
                    // self.current_direction = Direction::North;
                    self.current_coords.y += *by as isize;
                }
                East(by) => {
                    // self.current_direction = Direction::East;
                    self.current_coords.x += *by as isize;
                }
                South(by) => {
                    // self.current_direction = Direction::South;
                    self.current_coords.y -= *by as isize;
                }
                West(by) => {
                    // self.current_direction = Direction::West;
                    self.current_coords.x -= *by as isize;
                }
                Forward(by) => match self.current_direction {
                    Direction::North => self.current_coords.y += *by as isize,
                    Direction::East => self.current_coords.x += *by as isize,
                    Direction::South => self.current_coords.y -= *by as isize,
                    Direction::West => self.current_coords.x -= *by as isize,
                },
                Rotate(to, by) => self.current_direction = self.current_direction.rotate(to, by),
            }
        }
    }
}

struct WaypointInterpreter {
    waypoint: WayPoint,
    current_coords: Coords,
    instructions: Vec<Action>,
}

impl WaypointInterpreter {
    fn new(instructions: Vec<Action>) -> WaypointInterpreter {
        WaypointInterpreter {
            waypoint: WayPoint { x: 10, y: 1 },
            current_coords: Coords { x: 0, y: 0 },
            instructions,
        }
    }

    fn run(&mut self) {
        for action in self.instructions.iter() {
            match action {
                North(by) => {
                    // self.current_direction = Direction::North;
                    self.waypoint.move_north(*by);
                }
                East(by) => {
                    // self.current_direction = Direction::East;
                    self.waypoint.move_east(*by);
                }
                South(by) => {
                    // self.current_direction = Direction::South;
                    self.waypoint.move_south(*by);
                }
                West(by) => {
                    // self.current_direction = Direction::West;
                    self.waypoint.move_west(*by);
                }
                Forward(by) => {
                    let x_diff = *by as isize * self.waypoint.x;
                    let y_diff = *by as isize * self.waypoint.y;
                    self.current_coords.x += x_diff;
                    self.current_coords.y += y_diff;
                }
                Rotate(to, by) => self.waypoint.rotate(to, by),
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct WayPoint {
    // units relative to the ship
    x: isize,
    y: isize,
}

impl WayPoint {
    fn move_north(&mut self, amt: usize) {
        self.y += amt as isize;
    }

    fn move_east(&mut self, amt: usize) {
        self.x += amt as isize;
    }

    fn move_south(&mut self, amt: usize) {
        self.y -= amt as isize;
    }

    fn move_west(&mut self, amt: usize) {
        self.x -= amt as isize;
    }

    fn rotate(&mut self, to: &RotateTo, by: &Degrees) {
        // right-biased rotation
        let rotate_multiplier = if to == &RotateTo::Right { -1 } else { 1 };
        match *by {
            Degrees::_90 => {
                //
                let temp_y = self.y;
                self.y = rotate_multiplier * self.x;
                self.x = -rotate_multiplier * temp_y;
            }
            Degrees::_180 => {
                self.x = -self.x;
                self.y = -self.y;
            }
            Degrees::_270 => {
                let temp_y = self.y;
                self.y = -rotate_multiplier * self.x;
                self.x = rotate_multiplier * temp_y
            }
        }
    }
}

fn parse(s: &str) -> StdResult<Vec<Action>, easy::ParseError<&str>> {
    let north_parser = char('N').with(number_parser()).map(North);
    let east_parser = char('E').with(number_parser()).map(East);
    let south_parser = char('S').with(number_parser()).map(South);
    let west_parser = char('W').with(number_parser()).map(West);
    let forward_parser = char('F').with(number_parser()).map(Forward);
    let left_parser = char('L')
        .with(degrees_parser())
        .map(|degrees| Rotate(RotateTo::Left, degrees));
    let right_parser = char('R')
        .with(degrees_parser())
        .map(|degrees| Rotate(RotateTo::Right, degrees));

    let row_parser = choice!(
        north_parser,
        east_parser,
        south_parser,
        west_parser,
        forward_parser,
        left_parser,
        right_parser
    );

    let mut parser = many(row_parser.skip(newline()));
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

fn degrees_parser<Input>() -> impl Parser<Input, Output = Degrees>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    choice!(
        string("90").map(|_| _90),
        string("180").map(|_| _180),
        string("270").map(|_| _270)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use RotateTo::*;

    #[test]
    fn parse_simple_test() {
        let input = "F10
N3
F7
R90
F11
";
        let r = parse(input).unwrap();
        let expected = vec![
            Forward(10),
            North(3),
            Forward(7),
            Rotate(Right, _90),
            Forward(11),
        ];
        assert_eq!(expected, r);
    }

    #[test]
    fn direction_rotate_test() {
        use Direction::*;
        assert_eq!(East, North.rotate(&Right, &_90));
        assert_eq!(South, North.rotate(&Right, &_180));
        assert_eq!(West, North.rotate(&Right, &_270));
        assert_eq!(West, North.rotate(&Left, &_90));
        assert_eq!(South, North.rotate(&Left, &_180));
        assert_eq!(East, North.rotate(&Left, &_270));
    }

    #[test]
    fn simple_simulation_test() {
        let instructions = vec![
            Forward(10),
            North(3),
            Forward(7),
            Rotate(Right, _90),
            Forward(11),
        ];
        let mut simulation = Interpreter::new(instructions);
        simulation.run();
        assert_eq!(Coords { x: 17, y: -8 }, simulation.current_coords)
    }

    #[test]
    fn manhattan_distance_test() {
        assert_eq!(25, Coords { x: 17, y: -8 }.manhattan_distance())
    }

    #[test]
    fn way_point_rotate_test() {
        let mut way_point = WayPoint { x: 2, y: 1 };
        way_point.rotate(&Right, &_90);
        assert_eq!(WayPoint { x: 1, y: -2 }, way_point);
        way_point.rotate(&Right, &_90);
        assert_eq!(WayPoint { x: -2, y: -1 }, way_point);
        way_point.rotate(&Right, &_90);
        assert_eq!(WayPoint { x: -1, y: 2 }, way_point);
        way_point.rotate(&Right, &_180);
        assert_eq!(WayPoint { x: 1, y: -2 }, way_point);
        way_point.rotate(&Left, &_180);
        assert_eq!(WayPoint { x: -1, y: 2 }, way_point);
        way_point.rotate(&Left, &_270);
        assert_eq!(WayPoint { x: 2, y: 1 }, way_point);
        way_point.rotate(&Left, &_90);
        assert_eq!(WayPoint { x: -1, y: 2 }, way_point);
    }

    #[test]
    fn simple_way_pointsimulation_test() {
        let instructions = vec![
            Forward(10),
            North(3),
            Forward(7),
            Rotate(Right, _90),
            Forward(11),
        ];
        let mut simulation = WaypointInterpreter::new(instructions);
        simulation.run();
        assert_eq!(Coords { x: 214, y: -72 }, simulation.current_coords)
    }
}
