use anyhow::Result;
use combine::easy;
use combine::lib::collections::{HashMap, HashSet};
use combine::parser::char::*;
use combine::*;
use std::collections::hash_map::Entry;
use std::convert::TryInto;
use std::result::Result as StdResult;

const INPUT: &str = include_str!("../data/day_17_input");

pub fn run() -> Result<()> {
    println!("*** Day 17: Conway Cubes ***");
    println!("Input: {}", INPUT);
    let init_grid = parse(INPUT)?;
    let three_d = ThreeDimensionalGrid::from_initial_grid(&init_grid);
    let solution_1 = three_d.solution_1();

    println!("Solution 1: {:?}", solution_1);

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum State {
    On,
    Off,
}

#[derive(Debug, PartialEq, Eq)]
struct InitialGrid(Vec<Vec<State>>);

#[derive(Debug, PartialEq, Eq)]
struct ThreeDimensionalGrid {
    // (x, y z): If there is an entry in the HashSet, a given point is *on*
    on_states: HashMap<i64, HashMap<i64, HashSet<i64>>>,
}

fn turn_on(
    hash: &mut HashMap<i64, HashMap<i64, HashSet<i64>>>,
    x_idx: i64,
    y_idx: i64,
    z_idx: i64,
) {
    match hash.entry(x_idx as i64) {
        Entry::Occupied(ys) => match ys.into_mut().entry(y_idx) {
            Entry::Occupied(y_entry) => {
                let existing_set = y_entry.into_mut();
                existing_set.insert(z_idx);
            }
            Entry::Vacant(empty) => {
                let mut first_z_set = HashSet::new();
                first_z_set.insert(z_idx);
                empty.insert(first_z_set);
            }
        },
        Entry::Vacant(empty) => {
            let mut first_y_map = HashMap::new();
            let mut first_z_set = HashSet::new();
            first_z_set.insert(z_idx);
            first_y_map.insert(y_idx, first_z_set);
            empty.insert(first_y_map);
        }
    }
}

impl ThreeDimensionalGrid {
    fn from_initial_grid(init_grid: &InitialGrid) -> ThreeDimensionalGrid {
        let mut on_states: HashMap<i64, HashMap<i64, HashSet<i64>>> = HashMap::new();

        for (y_idx, row) in init_grid.0.iter().enumerate() {
            for (x_idx, state) in row.iter().enumerate() {
                if state == &State::On {
                    turn_on(&mut on_states, x_idx as i64, -(y_idx as i64), 0);
                }
            }
        }

        ThreeDimensionalGrid { on_states }
    }

    fn turn_on(&mut self, x_idx: i64, y_idx: i64, z_idx: i64) {
        turn_on(&mut self.on_states, x_idx, y_idx, z_idx);
    }

    fn turn_off(&mut self, x_idx: i64, y_idx: i64, z_idx: i64) {
        match self.on_states.entry(x_idx as i64) {
            Entry::Occupied(existing_ys) => match existing_ys.into_mut().entry(y_idx) {
                Entry::Occupied(y) => {
                    let existing_set = y.into_mut();
                    existing_set.remove(&z_idx);
                }
                Entry::Vacant(_) => (),
            },
            Entry::Vacant(_) => (),
        }
    }

    fn solution_1(mut self) -> usize {
        self.step_times(6);
        self.count_on_coords()
    }

    fn step_times(&mut self, times: usize) {
        for _ in 0..times {
            self.step();
        }
    }

    fn count_on_coords(&self) -> usize {
        self.on_states
            .iter()
            .map(|(_, ys)| {
                let on_zs: usize = ys.iter().map(|(_, zs)| zs.len()).sum();
                on_zs
            })
            .sum()
    }

    fn step(&mut self) {
        let current_ons = self.on_states.clone();

        for (x, ys) in current_ons.iter() {
            for (y, zs) in ys.iter() {
                for z in zs.iter() {
                    let mut point_touches = neighbours(*x, *y, *z).to_vec();
                    point_touches.push((*x, *y, *z));
                    // check if neighbours need to be turned off /o n
                    for (neighbour_x, neighbour_y, neighbour_z) in point_touches.iter() {
                        let is_neighbour_on =
                            is_on(&current_ons, *neighbour_x, *neighbour_y, *neighbour_z);
                        let neighbours_of_neighbours_on =
                            neighbours(*neighbour_x, *neighbour_y, *neighbour_z)
                                .iter()
                                .filter(
                                    |(
                                        neighbour_of_neighbour_x,
                                        neighbour_of_neighbour_y,
                                        neighbour_of_neighbour_z,
                                    )| {
                                        is_on(
                                            &current_ons,
                                            *neighbour_of_neighbour_x,
                                            *neighbour_of_neighbour_y,
                                            *neighbour_of_neighbour_z,
                                        )
                                    },
                                )
                                .count();
                        if is_neighbour_on {
                            if neighbours_of_neighbours_on == 2 || neighbours_of_neighbours_on == 3
                            {
                                self.turn_on(*neighbour_x, *neighbour_y, *neighbour_z);
                            } else {
                                self.turn_off(*neighbour_x, *neighbour_y, *neighbour_z);
                            }
                        } else {
                            if neighbours_of_neighbours_on == 3 {
                                self.turn_on(*neighbour_x, *neighbour_y, *neighbour_z);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn is_on(
    map: &HashMap<i64, HashMap<i64, HashSet<i64>>>,
    x_idx: i64,
    y_idx: i64,
    z_idx: i64,
) -> bool {
    map.get(&x_idx)
        .and_then(|ys| ys.get(&y_idx).filter(|z| z.contains(&z_idx)))
        .is_some()
}

fn neighbours(x: i64, y: i64, z: i64) -> [(i64, i64, i64); 26] {
    let vec: Vec<_> = (x - 1..=x + 1)
        .into_iter()
        .flat_map(move |neighbour_x| {
            (y - 1..=y + 1).into_iter().flat_map(move |neighbour_y| {
                (z - 1..=z + 1).into_iter().filter_map(move |neighbour_z| {
                    if (x, y, z) != (neighbour_x, neighbour_y, neighbour_z) {
                        Some((neighbour_x, neighbour_y, neighbour_z))
                    } else {
                        None
                    }
                })
            })
        })
        .collect();
    vec.try_into().expect("maths")
}

fn parse(s: &str) -> StdResult<InitialGrid, easy::ParseError<&str>> {
    let state_parser = choice!(char('#').map(|_| State::On), char('.').map(|_| State::Off));

    let row_parser = many(state_parser);
    let mut initial_grid_parser = many(row_parser.skip(newline())).map(InitialGrid);
    let (r, _) = initial_grid_parser.easy_parse(s)?;
    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;
    use State::*;

    #[test]
    fn parse_test() {
        let input = "#.#####.
#..##...
";
        let r = parse(input).unwrap();
        let expected = InitialGrid(vec![
            vec![On, Off, On, On, On, On, On, Off],
            vec![On, Off, Off, On, On, Off, Off, Off],
        ]);
        assert_eq!(expected, r);
    }

    #[test]
    fn threed_grid_init_test() {
        let initial_grid = InitialGrid(vec![
            vec![On, Off, On, On, On, On, On, Off],
            vec![On, Off, Off, On, On, Off, Off, Off],
        ]);

        let expected = {
            let mut x_map = HashMap::new();

            let mut z_zero_set = HashSet::new();
            z_zero_set.insert(0);

            let mut y_x_0_hash = HashMap::new();
            y_x_0_hash.insert(0, z_zero_set.clone());
            y_x_0_hash.insert(-1, z_zero_set.clone());
            x_map.insert(0, y_x_0_hash);

            let mut y_x_2_hash = HashMap::new();
            y_x_2_hash.insert(0, z_zero_set.clone());
            x_map.insert(2, y_x_2_hash);

            let mut y_x_3_hash = HashMap::new();
            y_x_3_hash.insert(0, z_zero_set.clone());
            y_x_3_hash.insert(-1, z_zero_set.clone());
            x_map.insert(3, y_x_3_hash);

            let mut y_x_4_hash = HashMap::new();
            y_x_4_hash.insert(0, z_zero_set.clone());
            y_x_4_hash.insert(-1, z_zero_set.clone());
            x_map.insert(4, y_x_4_hash);

            let mut y_x_5_hash = HashMap::new();
            y_x_5_hash.insert(0, z_zero_set.clone());
            x_map.insert(5, y_x_5_hash);

            let mut y_x_6_hash = HashMap::new();
            y_x_6_hash.insert(0, z_zero_set.clone());
            x_map.insert(6, y_x_6_hash);

            ThreeDimensionalGrid { on_states: x_map }
        };
        let r = ThreeDimensionalGrid::from_initial_grid(&initial_grid);
        assert_eq!(expected, r);
    }

    #[test]
    fn neighbours_test() {
        let expected = [
            // x at -1
            (-1, -1, -1),
            (-1, -1, 0),
            (-1, -1, 1),
            (-1, 0, -1),
            (-1, 0, 0),
            (-1, 0, 1),
            (-1, 1, -1),
            (-1, 1, 0),
            (-1, 1, 1),
            // x at 0
            (0, -1, -1),
            (0, -1, 0),
            (0, -1, 1),
            (0, 0, -1),
            // (0, 0, 0), filtered out
            (0, 0, 1),
            (0, 1, -1),
            (0, 1, 0),
            (0, 1, 1),
            // x at 1
            (1, -1, -1),
            (1, -1, 0),
            (1, -1, 1),
            (1, 0, -1),
            (1, 0, 0),
            (1, 0, 1),
            (1, 1, -1),
            (1, 1, 0),
            (1, 1, 1),
        ];
        assert_eq!(expected, neighbours(0, 0, 0));
    }

    #[test]
    fn solution_1_test() {
        let init = parse(
            ".#.
..#
###
",
        )
        .unwrap();
        let three_d = ThreeDimensionalGrid::from_initial_grid(&init);
        let r = three_d.solution_1();
        assert_eq!(112, r);
    }
}
