use anyhow::Result;
use combine::easy;
use combine::lib::fmt::Formatter;
use combine::parser::char::*;
use combine::*;
use std::result::Result as StdResult;

const INPUT: &str = include_str!("../data/day_11_input");

pub fn run() -> Result<()> {
    println!("*** Day 11: Seating System ***");
    println!("Input: {}", INPUT);
    let grid = parse(INPUT)?;

    let mut simulation = Simulation::new(grid.clone());
    simulation.run_til_no_changes(|s| s.step(4, |grid, i, j| grid.adjacent_grid_results(i, j)));
    println!("Solution 1: {:?}", simulation.occupied_seats());

    let mut simulation_tolerant = Simulation::new(grid.clone());
    simulation_tolerant
        .run_til_no_changes(|s| s.step(5, |grid, i, j| grid.next_adjacent_seat_results(i, j)));
    println!("Solution 2: {:?}", simulation_tolerant.occupied_seats());

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

    // Non-wrap
    fn next_adjacent_seat_results(&self, i: usize, j: usize) -> Vec<&Space> {
        let north = if 0 == i {
            None
        } else {
            (0..i)
                .rev()
                .filter_map(|row_idx| {
                    self.0
                        .get(row_idx)
                        .and_then(|row| row.get(j))
                        .filter(|space| **space != Space::Floor)
                })
                .next()
        };
        let north_east = if 0 == i {
            None
        } else {
            (0..i)
                .rev()
                .enumerate()
                .filter_map(|(step_idx, row_idx)| {
                    let col = j + step_idx + 1;
                    self.0
                        .get(row_idx)
                        .and_then(|row| row.get(col))
                        .filter(|space| **space != Space::Floor)
                })
                .next()
        };
        let east = {
            self.0.get(i).and_then(|current_row| {
                current_row
                    .iter()
                    .skip(j + 1)
                    .filter(|space| **space != Space::Floor)
                    .next()
            })
        };

        let south_east = {
            self.0
                .iter()
                .skip(i + 1)
                .enumerate()
                .filter_map(|(step_idx, row)| {
                    row.get(j + step_idx + 1)
                        .filter(|space| **space != Space::Floor)
                })
                .next()
        };

        let south = {
            self.0
                .iter()
                .skip(i + 1)
                .filter_map(|row| row.get(j).filter(|space| **space != Space::Floor))
                .next()
        };

        let south_west = if j == 0 {
            None
        } else {
            self.0
                .iter()
                .skip(i + 1)
                .zip((0..j).rev())
                .filter_map(|(south_row, col)| {
                    south_row.get(col).filter(|space| **space != Space::Floor)
                })
                .next()
        };

        let west = if j == 0 {
            None
        } else {
            self.0.get(i).and_then(|row| {
                row.iter()
                    .take(j)
                    .rev()
                    .filter(|space| **space != Space::Floor)
                    .next()
            })
        };

        let north_west = if i == 0 || j == 0 {
            None
        } else {
            (0..i)
                .rev()
                .zip((0..j).rev())
                .filter_map(|(row_idx, col_idx)| {
                    self.0
                        .get(row_idx)
                        .and_then(|row| row.get(col_idx))
                        .filter(|space| **space != Space::Floor)
                })
                .next()
        };

        let before_flattening = vec![
            north, north_east, east, south_east, south, south_west, west, north_west,
        ];
        before_flattening.into_iter().flatten().collect()
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
    //
    // fn step(&mut self) {
    //     self.step2(|grid, i, j| grid.adjacent_grid_results(i, j))
    // }
    //
    // fn step_tolerant(&mut self) {
    //     self.step2(|grid, i, j| grid.next_adjacent_seat_results(i, j))
    // }

    fn step<F>(&mut self, max_occupied_seats: usize, adjacent_search: F)
    where
        F: Fn(&Grid, usize, usize) -> Vec<&Space>,
    {
        for (i, row) in self.current.0.iter().enumerate() {
            for (j, space) in row.iter().enumerate() {
                if *space != Space::Floor {
                    let adjacent_occupied_seats = adjacent_search(&self.current, i, j)
                        .iter()
                        .filter(|s| ***s == Space::Occupied)
                        .count();
                    if *space == Space::EmptySeat && adjacent_occupied_seats == 0 {
                        self.next.0[i][j] = Space::Occupied
                    } else if *space == Space::Occupied
                        && adjacent_occupied_seats >= max_occupied_seats
                    {
                        self.next.0[i][j] = Space::EmptySeat
                    }
                }
            }
        }
        self.no_changes = self.current == self.next;
        self.current = self.next.clone();
    }

    fn run_til_no_changes<F>(&mut self, step: F)
    where
        F: Fn(&mut Self) -> (),
    {
        while !self.no_changes {
            step(self);
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
            vec![&EmptySeat, &Floor, &EmptySeat,],
            grid.adjacent_grid_results(2, 9)
        );
    }

    #[test]
    fn run_till_end_and_count_occupied_test() {
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
        grid.run_til_no_changes(|s| s.step(4, |grid, i, j| grid.adjacent_grid_results(i, j)));
        assert_eq!(37, grid.occupied_seats());
    }

    #[test]
    fn next_adjacent_seat_results_1_test() {
        let input = ".......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....
";
        let parsed = parse(input).unwrap();
        let r = parsed.next_adjacent_seat_results(4, 3);
        let expected = vec![
            &Occupied, &Occupied, &Occupied, &Occupied, &Occupied, &Occupied, &Occupied, &Occupied,
        ];
        assert_eq!(expected, r)
    }

    #[test]
    fn next_adjacent_seat_results_2_test() {
        let input = ".............
.L.L.#.#.#.#.
.............
";
        let parsed = parse(input).unwrap();
        let r = parsed.next_adjacent_seat_results(1, 1);
        let expected = vec![&EmptySeat];
        assert_eq!(expected, r)
    }
    #[test]
    fn next_adjacent_seat_results_3_test() {
        let input = ".##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.
";
        let parsed = parse(input).unwrap();
        let r = parsed.next_adjacent_seat_results(3, 3);
        let expected: Vec<&Space> = vec![];
        assert_eq!(expected, r)
    }

    #[test]
    fn run_till_end_tolerant_and_count_occupied_test() {
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
        grid.run_til_no_changes(|s| s.step(5, |grid, i, j| grid.next_adjacent_seat_results(i, j)));
        assert_eq!(26, grid.occupied_seats());
    }
}
