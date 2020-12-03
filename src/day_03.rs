use combine::easy::ParseError;
use combine::parser::char::*;
use combine::*;
use std::result::Result as StdResult;

const INPUT: &str = include_str!("../data/day_03_input");

struct Trajectory {
    right: usize,
    down: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum Space {
    Tree,
    Open,
}

#[derive(Debug, PartialEq, Eq)]
struct Row(Vec<Space>);

#[derive(Debug, PartialEq, Eq)]
struct Map(Vec<Row>);

fn parse(s: &str) -> StdResult<Map, ParseError<&str>> {
    let space_parser = char('.').map(|_| Space::Open);
    let tree_parser = char('#').map(|_| Space::Tree);
    let row_parser = many(space_parser.or(tree_parser)).map(Row);
    let mut map_parser = many(row_parser.skip(newline())).map(Map);
    let (r, _) = map_parser.easy_parse(s)?;
    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;
    use Space::*;

    #[test]
    fn test_parse() {
        let test_input = "..##.......
#...#...#..
";
        let r = parse(test_input).unwrap();
        let expected = Map(vec![
            Row(vec![
                Open, Open, Tree, Tree, Open, Open, Open, Open, Open, Open, Open,
            ]),
            Row(vec![
                Tree, Open, Open, Open, Tree, Open, Open, Open, Tree, Open, Open,
            ]),
        ]);
        assert_eq!(expected, r);
    }
}
