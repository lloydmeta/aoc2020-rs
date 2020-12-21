use std::collections::HashMap;

use crate::day_20::NeighbourRelativeToSelf::*;
use anyhow::{Context, Result};
use combine::lib::collections::HashSet;
use Manipulate::*;

use combine::easy;
use combine::parser::char::*;
use combine::*;
use itertools::Itertools;
use std::num::ParseIntError;
use std::result::Result as StdResult;
use std::convert::TryInto;

const INPUT: &str = include_str!("../data/day_20_input");

/// The complete cycle of matrix manipulations that cover all unique transformations
static MATRIX_MANIPULATIONS: [Manipulate; 9] = [
    RotateRight,
    RotateRight,
    RotateRight,
    RotateRight,
    FlipHorizontal,
    RotateRight,
    RotateRight,
    RotateRight,
    FlipHorizontal,
];

type Coords = (i64, i64); // (x, y)

pub fn run() -> Result<()> {
    println!("*** Day 20: Jurassic Jigsaw ***");
    println!("Input: {}", INPUT);

    let mut image = parse(INPUT)?;
    let sol_1 = solution_1(&mut image)?;

    println!("Solution 1: {}", sol_1);

    Ok(())
}

fn solution_1(img: &mut OverallImage) -> Result<usize> {
    img.solve()?;
    Ok(img.corner_tiles()?.iter().map(|(idx, _)| *idx).product())
}

fn parse(s: &str) -> StdResult<OverallImage, easy::ParseError<&str>> {
    let tile_idx_to_image =
        s.trim()
            .split("\n\n")
            .try_fold(HashMap::new(), |mut acc, tile_str| {
                let mut idx_with_image_tile_parser = tile_with_idx_parser();
                let ((idx, image), _) = idx_with_image_tile_parser.easy_parse(tile_str)?;
                acc.insert(idx, image);

                Ok(acc)
            })?;
    Ok(OverallImage {
        tiles: tile_idx_to_image,
        coords_to_tile_idx: HashMap::new(),
    })
}

fn tile_with_idx_parser<Input>() -> impl Parser<Input, Output=(usize, ImageTile)>
    where
        Input: Stream<Token=char>,
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
    let tile_idx_parser = string("Tile ").with(number()).skip(char(':'));
    let pixel_parser = char('#')
        .map(|_| Pixel::On)
        .or(char('.').map(|_| Pixel::Off));
    let row_parser = many(pixel_parser);
    let matrix_parser = sep_by1(row_parser, newline()).map(|vec: Vec<Vec<Pixel>>| ImageTile {
        image: MonochromeSquare(vec),
        coords: None,
        neighbours_indices: None,
    });
    tile_idx_parser.skip(newline()).and(matrix_parser)
}

fn number<Input>() -> impl Parser<Input, Output=usize>
    where
        Input: Stream<Token=char>,
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

struct OverallImage {
    tiles: HashMap<usize, ImageTile>,

    coords_to_tile_idx: HashMap<Coords, usize>,
}

impl OverallImage {
    fn corner_tiles(&self) -> Result<[(&usize, &ImageTile); 4]> {
        let v: Vec<_> = self.tiles
            .iter()
            .filter(|(_, tile)| {
                if let Some(neighbours) = &tile.neighbours_indices {
                    neighbours.len() == 2
                } else {
                    false
                }
            })
            .sorted_by_key(|(idx, _)| **idx)
            .collect();
        let arr_or_err: Result<[(&usize, &ImageTile); 4], _> = v.try_into();
        match arr_or_err {
            Ok(r) => Ok(r),
            Err(e) => anyhow::bail!("This was not a 4 element thing [{:?}]", e)
        }
    }

    fn solve(&mut self) -> Result<()> {
        if self.tiles.len() == self.coords_to_tile_idx.len() {
            debug!("Skipping .. already solved");
        } else if let Some((first_idx, tile)) = self
            .tiles
            .iter_mut()
            .find(|(_, tile)| tile.coords.is_none() && tile.neighbours_indices.is_none())
        {
            let init_coords = (0, 0);
            self.coords_to_tile_idx.insert(init_coords, *first_idx);
            tile.coords = Some(init_coords);
            let mut queue = vec![*first_idx];
            debug!("about to start inner solve");
            self.inner_solve(&mut queue)?
        }
        Ok(())
    }

    fn inner_solve(&mut self, tile_idx_queue: &mut Vec<usize>) -> Result<()> {
        while let Some(current_tile_idx) = tile_idx_queue.pop() {
            debug!("working on {}", current_tile_idx);
            let tile_under_consideration_non_mut = self
                .tiles
                .get(&current_tile_idx)
                .context("Tile should exist")?;

            // We can safely assume that coords for the current tile has already been assigned
            // because we assign coords to neighbours who have been identified before putting
            // those neighbours in the queue to be loked up

            let current_tile_all_sides_with_neighbour_idx_and_required_side_patterns: Vec<_> =
                [Top, Right, Bottom, Left]
                    .iter()
                    .filter_map(|relative_to_self_edge_name| {
                        Some((
                            *relative_to_self_edge_name,
                            tile_under_consideration_non_mut
                                .neighbour_coords_with_edges(*relative_to_self_edge_name)?,
                        ))
                    })
                    .collect();

            let mut current_tile_neighbours_map = HashMap::new();

            // For edges that are *already* assigned to a tile, assign them as neighbours to this current tile
            // and remove them from the search space
            let unassigned_tile_sides_with_neighbour_idx_and_required_side_patterns: Vec<_> =
                current_tile_all_sides_with_neighbour_idx_and_required_side_patterns
                    .into_iter()
                    .filter_map(
                        |(relative_to_self_edge_name, (neighbour_coords, self_edge_pattern))| {
                            if let Some(already_placed_tile_idx) =
                            self.coords_to_tile_idx.get(&neighbour_coords)
                            {
                                debug!(
                                    "found already-placed [{:?}] neighbour [{}] for [{}]",
                                    relative_to_self_edge_name,
                                    already_placed_tile_idx,
                                    current_tile_idx
                                );
                                current_tile_neighbours_map
                                    .insert(relative_to_self_edge_name, *already_placed_tile_idx);

                                None // remove this edge from having to be considered
                            } else {
                                Some((
                                    relative_to_self_edge_name,
                                    (neighbour_coords, self_edge_pattern),
                                ))
                            }
                        },
                    )
                    .collect();

            'tile_candidate: for (tile_candidate_idx, tile_candidate) in self.tiles.iter_mut() {
                if *tile_candidate_idx != current_tile_idx // Not current
                    // has not yet been considered: prevent us from manipulating already-placed tiles
                    && !current_tile_neighbours_map.values().collect::<HashSet<_>>().contains(tile_candidate_idx)
                    && tile_candidate.coords.is_none()
                /* means it hasn't been processed*/
                {
                    for image_manipulation in MATRIX_MANIPULATIONS.iter() {
                        tile_candidate.image.modify_image(image_manipulation);
                        for (self_side, (neighbour_coords, self_edge)) in
                        unassigned_tile_sides_with_neighbour_idx_and_required_side_patterns
                            .iter()
                        {
                            if !current_tile_neighbours_map.contains_key(&self_side) {
                                let candidate_side_name = match self_side {
                                    Top => Bottom,
                                    Right => Left,
                                    Bottom => Top,
                                    Left => Right,
                                };
                                let candidate_edge = tile_candidate
                                    .image
                                    .get_edge(&candidate_side_name)
                                    .context("Edge does not exist on candidate")?;
                                if candidate_edge == *self_edge {
                                    // Set as neighbour of current time
                                    current_tile_neighbours_map
                                        .insert(*self_side, *tile_candidate_idx);

                                    // Set coords for neighbour
                                    tile_candidate.coords = Some(*neighbour_coords);
                                    self.coords_to_tile_idx
                                        .insert(*neighbour_coords, *tile_candidate_idx);

                                    // Add to queue to be considered later
                                    tile_idx_queue.push(*tile_candidate_idx);

                                    // no longer consider this candidate: it can only be on one side relative to this current time
                                    continue 'tile_candidate;
                                }
                            }
                        }
                    }
                }
                if current_tile_neighbours_map.len() == 4 {
                    break 'tile_candidate;
                }
            }
            let tile_under_consideration_mut = self
                .tiles
                .get_mut(&current_tile_idx)
                .context("Tile should exist")?;
            debug!(
                "neighbours for [{}]: [{:?}]",
                current_tile_idx, current_tile_neighbours_map
            );
            tile_under_consideration_mut.neighbours_indices = Some(current_tile_neighbours_map);
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum NeighbourRelativeToSelf {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Debug, PartialEq)]
struct ImageTile {
    image: MonochromeSquare,

    coords: Option<Coords>,

    // [ top, right, bottom, left]
    neighbours_indices: Option<HashMap<NeighbourRelativeToSelf, usize>>,
}

impl ImageTile {
    fn neighbour_coords_with_edges(
        &self,
        relative_to_self: NeighbourRelativeToSelf,
    ) -> Option<(Coords, Vec<Pixel>)> {
        let (self_x, self_y) = self.coords?;
        let neighbour_coords_with_edge = match relative_to_self {
            Top => ((self_x, self_y + 1), self.image.get_edge(&Top)?),
            Right => ((self_x + 1, self_y), self.image.get_edge(&Right)?),
            Bottom => ((self_x, self_y - 1), self.image.get_edge(&Bottom)?),
            Left => ((self_x - 1, self_y), self.image.get_edge(&Left)?),
        };
        Some(neighbour_coords_with_edge)
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Hash)]
enum Pixel {
    On,
    Off,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Hash)]
enum Manipulate {
    RotateRight,
    FlipHorizontal,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq)]
struct MonochromeSquare(Vec<Vec<Pixel>>);

impl MonochromeSquare {
    fn get_edge(&self, rel: &NeighbourRelativeToSelf) -> Option<Vec<Pixel>> {
        match *rel {
            Top => self.0.first().cloned(),
            Right => {
                let t = self
                    .0
                    .iter()
                    .filter_map(|row| row.last().copied())
                    .collect();
                Some(t)
            }
            Bottom => self.0.last().cloned(),
            Left => {
                let t = self
                    .0
                    .iter()
                    .filter_map(|row| row.first().copied())
                    .collect();
                Some(t)
            }
        }
    }
    fn modify_image(&mut self, manipulation: &Manipulate) {
        match *manipulation {
            Manipulate::RotateRight => rotate_right_square(&mut self.0),
            Manipulate::FlipHorizontal => flip_horizontal_square(&mut self.0),
        }
    }
}

fn rotate_right_square<X: Copy>(mat: &mut Vec<Vec<X>>) {
    let length = mat.len();
    let cycles = length / 2;
    for i in 0..cycles {
        for j in i..(length - i - 1) {
            // temp variable
            let temp = mat[i][j];
            mat[i][j] = mat[length - 1 - j][i];
            mat[length - 1 - j][i] = mat[length - 1 - i][length - 1 - j];
            mat[length - 1 - i][length - 1 - j] = mat[j][length - 1 - i];
            mat[j][length - 1 - i] = temp;
        }
    }
}

fn flip_horizontal_square<X>(mat: &mut Vec<Vec<X>>) {
    for row in mat.iter_mut() {
        row.reverse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_square_test() {
        let mut square = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        rotate_right_square(&mut square);

        let expected = vec![
            vec![13, 9, 5, 1],
            vec![14, 10, 6, 2],
            vec![15, 11, 7, 3],
            vec![16, 12, 8, 4],
        ];
        assert_eq!(expected, square)
    }

    #[test]
    fn flip_horizontal_square_test() {
        let mut square = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        flip_horizontal_square(&mut square);

        let expected = vec![
            vec![4, 3, 2, 1],
            vec![8, 7, 6, 5],
            vec![12, 11, 10, 9],
            vec![16, 15, 14, 13],
        ];
        assert_eq!(expected, square)
    }

    #[test]
    fn parse_test() {
        let r = parse(TEST_INPUT).unwrap();
        assert_eq!(9, r.tiles.len());

        for (_, tile) in r.tiles {
            assert_eq!(10, tile.image.0.len());
            for row in tile.image.0 {
                assert_eq!(10, row.len())
            }
        }
    }

    #[test]
    fn solution_1_test() {
        let mut r = parse(TEST_INPUT).unwrap();
        let r = solution_1(&mut r).unwrap();
        assert_eq!(20899048083289, r);
    }

    const TEST_INPUT: &str = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";
}
