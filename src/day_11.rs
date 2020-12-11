use anyhow::Result;
use combine::easy;
use combine::parser::char::*;
use combine::*;
use std::result::Result as StdResult;
use combine::lib::fmt::Formatter;

const INPUT: &str = include_str!("../data/day_11_input");

pub fn run() -> Result<()> {
    println!("*** Day 11: Seating System ***");
    println!("Input: {}", INPUT);
    let grid = parse(INPUT)?;


    let mut simulation = Simulation::new(grid);
    simulation.run_til_no_changes();
    println!("Solution 1: {:?}", simulation.occupied_seats());

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Space {
    Occupied,
    Floor,
    EmptySeat,
}

impl std::fmt::Display for Space {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Space::Occupied => "#",
            Space::EmptySeat => "L",
            Space::Floor => ".",
        };
        write!(f, "{}", char)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Grid(Vec<Vec<Space>>);


impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.0.iter() {
            for space in row {
                write!(f, "{}", space)?
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

impl Grid {
    // Non-wrap
    fn adjacent_grid_results(&self, i: usize, j: usize) -> Vec<&Space> {
        let north = if 0 == i {
            None
        } else {
            self.0.get(i - 1).and_then(|north_row| north_row.get(j))
        };
        let north_east = if 0 == i {
            None
        } else {
            self.0.get(i - 1).and_then(|north_row| north_row.get(j + 1))
        };
        let east = self.0.get(i).and_then(|current_row| current_row.get(j + 1));

        let south_east = self.0.get(i + 1).and_then(|south_row| south_row.get(j + 1));
        let south = self.0.get(i + 1).and_then(|south_row| south_row.get(j));

        let south_west = if j == 0 {
            None
        } else {
            self.0.get(i + 1).and_then(|south_row| south_row.get(j - 1))
        };

        let west = if j == 0 {
            None
        } else {
            self.0.get(i).and_then(|south_row| south_row.get(j - 1))
        };

        let north_west = if i == 0 || j == 0 {
            None
        } else {
            self.0.get(i - 1).and_then(|south_row| south_row.get(j - 1))
        };

        vec![
            north, north_east, east, south_east, south, south_west, west, north_west,
        ]
            .into_iter()
            .flatten()
            .collect()
    }

    fn occupied_seats(&self) -> usize {
        self.0.iter().fold(0, |acc, row| {
            row.iter().fold(acc, |inner_acc, space| {
                if *space == Space::Occupied {
                    inner_acc + 1
                } else {
                    inner_acc
                }
            })
        })
    }
}

fn parse(s: &str) -> StdResult<Grid, easy::ParseError<&str>> {
    let empty_seat_parser = char('L').map(|_| Space::EmptySeat);
    let occupied_parser = char('#').map(|_| Space::Occupied);
    let floor_parser = char('.').map(|_| Space::Floor);
    let space_parser = choice!(empty_seat_parser, occupied_parser, floor_parser);
    let row_parser = many(space_parser);
    let mut grid_parser = many(row_parser.skip(newline())).map(Grid);
    let (r, _) = grid_parser.easy_parse(s)?;
    Ok(r)
}

struct Simulation {
    current: Grid,
    next: Grid,
    no_changes: bool,
}

impl Simulation {

    fn occupied_seats(&self) -> usize {
        self.current.occupied_seats()
    }

    fn new(grid: Grid) -> Simulation {
        Simulation {
            current: grid.clone(),
            next: grid,
            no_changes: false,
        }
    }

    fn step(&mut self) {
        for (i, row) in self.current.0.iter().enumerate() {
            for (j, space) in row.iter().enumerate() {
                if *space != Space::Floor {
                    let adjacent_occupied_seats = self
                        .current
                        .adjacent_grid_results(i, j)
                        .iter()
                        .filter(|s| ***s == Space::Occupied)
                        .count();
                    if *space == Space::EmptySeat && adjacent_occupied_seats == 0 {
                        self.next.0[i][j] = Space::Occupied
                    } else if *space == Space::Occupied && adjacent_occupied_seats >= 4 {
                        self.next.0[i][j] = Space::EmptySeat
                    }
                }
            }
        }
        self.no_changes = self.current == self.next;
        self.current = self.next.clone();
    }

    fn run_til_no_changes(&mut self) {
        while !self.no_changes {
            println!("{}", self.current);
            self.step();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Space::*;

    #[test]
    fn parse_test() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
";
        let r = parse(input).unwrap();
        let expected = Grid(vec![
            vec![
                EmptySeat, Floor, EmptySeat, EmptySeat, Floor, EmptySeat, EmptySeat, Floor,
                EmptySeat, EmptySeat,
            ],
            vec![
                EmptySeat, EmptySeat, EmptySeat, EmptySeat, EmptySeat, EmptySeat, EmptySeat, Floor,
                EmptySeat, EmptySeat,
            ],
        ]);
        assert_eq!(expected, r);
    }

    #[test]
    fn adjacent_grid_results_test() {
        let grid = Grid(vec![
            vec![
                EmptySeat, Floor, EmptySeat, EmptySeat, Floor, EmptySeat, EmptySeat, Floor,
                EmptySeat, EmptySeat,
            ],
            vec![
                EmptySeat, EmptySeat, EmptySeat, EmptySeat, EmptySeat, EmptySeat, EmptySeat, Floor,
                EmptySeat, EmptySeat,
            ],
            vec![
                EmptySeat, Floor, EmptySeat, Floor, EmptySeat, Floor, Floor, EmptySeat, Floor,
                Floor,
            ],
        ]);

        assert_eq!(
            vec![&Floor, &EmptySeat, &EmptySeat],
            grid.adjacent_grid_results(0, 0)
        );
        assert_eq!(
            vec![
                &EmptySeat, &Floor, &EmptySeat, &EmptySeat, &Floor, &EmptySeat, &EmptySeat,
                &EmptySeat
            ],
            grid.adjacent_grid_results(1, 3)
        );
        assert_eq!(
            vec![&EmptySeat, &Floor, &EmptySeat, ],
            grid.adjacent_grid_results(2, 9)
        );
    }

    #[test]
    fn run_till_end_and_count_occupid_test() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";
        let mut grid = Simulation::new(parse(input).unwrap());
        grid.run_til_no_changes();
        assert_eq!(37, grid.occupied_seats());
    }
}
