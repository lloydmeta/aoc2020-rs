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
}
