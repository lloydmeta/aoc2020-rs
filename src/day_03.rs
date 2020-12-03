use anyhow::Result;
use combine::easy::ParseError;
use combine::parser::char::*;
use combine::*;
use itertools::Itertools;
use std::result::Result as StdResult;

const INPUT: &str = include_str!("../data/day_03_input");

pub fn run() -> Result<()> {
    println!("*** Day 3: Toboggan Trajectory ***");
    println!("Input: {}", INPUT);
    let map = parse(INPUT)?;
    let trajectory_1 = Trajectory { right: 3, down: 1 };
    let result_1 = count_trees_hit(&trajectory_1, &map);
    println!("Solution 1: {}", result_1);

    let all_trajectories = vec![
        Trajectory { right: 1, down: 1 },
        Trajectory { right: 3, down: 1 },
        Trajectory { right: 5, down: 1 },
        Trajectory { right: 7, down: 1 },
        Trajectory { right: 1, down: 2 },
    ];
    let trees_count_product = all_trajectories
        .iter()
        .map(|traj| count_trees_hit(traj, &map))
        .fold1(|acc, next| acc * next);
    println!("Solution 2: {:?}", trees_count_product);
    Ok(())
}

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

impl Map {
    /// Arboreal genetics and biome stability traverse
    /// Basically scrolls to the right infinitely, but not to the bottom of the map
    fn arboreal_space_at(&self, i: usize, j: usize) -> Option<&Space> {
        self.0.get(i).and_then(|Row(row)| {
            if row.len() <= j {
                let wrap_around = j % row.len();
                row.get(wrap_around)
            } else {
                row.get(j)
            }
        })
    }
}

fn parse(s: &str) -> StdResult<Map, ParseError<&str>> {
    let space_parser = char('.').map(|_| Space::Open);
    let tree_parser = char('#').map(|_| Space::Tree);
    let row_parser = many(space_parser.or(tree_parser)).map(Row);
    let mut map_parser = many(row_parser.skip(newline())).map(Map);
    let (r, _) = map_parser.easy_parse(s)?;
    Ok(r)
}

fn count_trees_hit(trajectory: &Trajectory, map: &Map) -> usize {
    (0..map.0.len())
        .step_by(trajectory.down)
        .zip((0..).step_by(trajectory.right))
        .into_iter()
        .fold(0, |acc, (i, j)| {
            if let Some(space) = map.arboreal_space_at(i, j) {
                if space == &Space::Tree {
                    acc + 1
                } else {
                    acc
                }
            } else {
                acc
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use Space::*;

    #[test]
    fn parse_test() {
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

    #[test]
    fn real_input_parse_test() {
        assert!(parse(INPUT).is_ok());
    }

    #[test]
    fn count_trees_hit_test() {
        let input = "..##.........##.........##.........##.........##.........##.......
#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....
.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........#.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...##....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#
";
        let map = parse(&input).unwrap();
        let trajectory = Trajectory { right: 3, down: 1 };
        let r = count_trees_hit(&trajectory, &map);
        assert_eq!(7, r);
    }

    #[test]
    fn map_arboreal_space_at_test() {
        let map = Map(vec![
            Row(vec![
                Open, Open, Tree, Tree, Open, Open, Open, Open, Open, Open, Open,
            ]),
            Row(vec![
                Tree, Open, Open, Open, Tree, Open, Open, Open, Tree, Open, Open,
            ]),
        ]);
        let space_at_1_0 = map.arboreal_space_at(1, 0);
        let space_at_1_12 = map.arboreal_space_at(1, 12);
        let space_at_2_12 = map.arboreal_space_at(2, 12);
        assert_eq!(Some(&Tree), space_at_1_0);
        assert_eq!(Some(&Open), space_at_1_12);
        assert_eq!(None, space_at_2_12);
    }
}
