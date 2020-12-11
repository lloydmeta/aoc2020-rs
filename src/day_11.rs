use combine::easy;
use combine::parser::char::*;
use combine::*;
use std::result::Result as StdResult;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Space {
    Occupied,
    Floor,
    EmptySeat,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Grid(Vec<Vec<Space>>);

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
}
