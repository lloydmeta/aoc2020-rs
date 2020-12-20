use std::collections::HashMap;

use crate::day_20::NeighbourRelativeToSelf::*;
use anyhow::{Context, Result};
use combine::lib::collections::HashSet;
use Manipulate::*;

/// The complete cycle of matrix manipulations that cover all unique transformations
static MATRIX_MANIPULATIONS: [Manipulate; 8] = [
    RotateRight,
    RotateRight,
    RotateRight,
    RotateRight,
    FlipHorizontal,
    RotateRight,
    RotateRight,
    RotateRight,
];

type Coords = (i64, i64); // (x, y)

struct OverallImage {
    tiles: HashMap<usize, ImageTile>,

    coords_to_tile_idx: HashMap<Coords, usize>,
}

impl OverallImage {
    fn solve(&mut self) -> Result<()> {
        if self.tiles.len() == self.coords_to_tile_idx.len() {
            ()
        } else if let Some((first_idx, tile)) = self
            .tiles
            .iter_mut()
            .find(|(_, tile)| tile.coords.is_none() && tile.neighbours_indices.is_none())
        {
            let init_coords = (0, 0);
            tile.coords = Some(init_coords);
            let mut queue = vec![*first_idx];
            self.inner_solve(&mut queue)?;
        }
        Ok(())
    }

    fn inner_solve(&mut self, tile_idx_queue: &mut Vec<usize>) -> Result<()> {
        while let Some(current_tile_idx_id) = tile_idx_queue.pop() {
            let tile_under_consideration_non_mut = self
                .tiles
                .get(&current_tile_idx_id)
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
                if *tile_candidate_idx != current_tile_idx_id // Not current
                    // has not yet been considered: prevent us from manipulating already-placed tiles
                    && current_tile_neighbours_map.values().collect::<HashSet<_>>().contains(tile_candidate_idx)
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
                .get_mut(&current_tile_idx_id)
                .context("Tile should exist")?;
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
            Left => ((self_x - 1, self_y - 1), self.image.get_edge(&Left)?),
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
    fn get_edges(&self) -> HashMap<NeighbourRelativeToSelf, Vec<Pixel>> {
        [Top, Right, Bottom, Right]
            .iter()
            .fold(HashMap::new(), |mut acc, next| {
                if let Some(edge) = self.get_edge(next) {
                    acc.insert(*next, edge);
                }
                acc
            })
    }

    fn get_edge(&self, rel: &NeighbourRelativeToSelf) -> Option<Vec<Pixel>> {
        match rel {
            &Top => self.0.first().cloned(),
            &Right => {
                let t = self
                    .0
                    .iter()
                    .filter_map(|row| row.last().copied())
                    .collect();
                Some(t)
            }
            &Bottom => self.0.last().cloned(),
            &Left => {
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
        match manipulation {
            &Manipulate::RotateRight => rotate_right_square(&mut self.0),
            &Manipulate::FlipHorizontal => flip_horizontal_square(&mut self.0),
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
    use Pixel::*;

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
}
