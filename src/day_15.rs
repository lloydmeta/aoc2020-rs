use anyhow::Result;
use combine::lib::collections::HashMap;

const INPUT: &str = include_str!("../data/day_15_input");

pub fn run() -> Result<()> {
    println!("*** Day 15: Rambunctious Recitation ***");
    println!("Input: {}", INPUT);
    let game = parse(INPUT)?;

    println!("Solution 1: {:?}", game.clone().nth_number(2020));

    // totally brute force
    println!("Solution 2: {:?}", game.nth_number(30000000));

    Ok(())
}

fn parse(s: &str) -> Result<Game> {
    let nums = s
        .trim()
        .split(',')
        .into_iter()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();
    Ok(Game::new(nums))
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
struct SpokenLast(usize);

#[derive(Debug, PartialEq, Clone)]
struct Game {
    numbers_to_spoken_on: HashMap<usize, SpokenLast>,

    initial_numbers: Vec<usize>,
    last_number: Option<usize>,
    current_idx: usize,
}

impl Game {
    fn new(initial_numbers: Vec<usize>) -> Game {
        let init_len = initial_numbers.len();
        let last_init_num = initial_numbers.last().copied();
        let mut numbers_to_spoken_on = HashMap::new();
        for (spoken_on_idx, num) in initial_numbers.iter().enumerate() {
            numbers_to_spoken_on.insert(*num, SpokenLast(spoken_on_idx));
        }
        Game {
            numbers_to_spoken_on,
            initial_numbers,
            current_idx: init_len - 1,
            last_number: last_init_num,
        }
    }

    fn step(&mut self) -> Option<usize> {
        let last_num = self.last_number?;
        let last_turn = self.current_idx;
        self.current_idx += 1;
        let result = if let Some(last_spoken) = self.numbers_to_spoken_on.get(&last_num).copied() {
            last_turn - last_spoken.0
        } else {
            0
        };

        self.last_number = Some(result);

        self.numbers_to_spoken_on
            .insert(last_num, SpokenLast(last_turn));
        Some(result)
    }

    fn nth_number(mut self, n: usize) -> Option<usize> {
        if n <= self.current_idx {
            None
        } else {
            while self.current_idx < n - 2 {
                self.step();
            }
            self.step()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let r = parse("11,0,1,10,5,19").unwrap();
        assert_eq!(vec![11, 0, 1, 10, 5, 19], r.initial_numbers);
    }

    #[test]
    fn step_test() {
        let mut game = Game::new(vec![0, 3, 6]);
        for expected in vec![0, 3, 3, 1, 0, 4, 0] {
            let r = game.step().unwrap();
            assert_eq!(expected, r);
        }
    }

    #[test]
    fn nth_number_test() {
        let game = Game::new(vec![0, 3, 6]);
        assert_eq!(0, game.clone().nth_number(4).unwrap());
        assert_eq!(3, game.clone().nth_number(6).unwrap());
        assert_eq!(1, game.clone().nth_number(7).unwrap());
        assert_eq!(0, game.clone().nth_number(8).unwrap());
        assert_eq!(4, game.clone().nth_number(9).unwrap());
        assert_eq!(0, game.clone().nth_number(10).unwrap());

        assert_eq!(1, Game::new(vec![1, 3, 2]).nth_number(2020).unwrap());
        assert_eq!(10, Game::new(vec![2, 1, 3]).nth_number(2020).unwrap());
        assert_eq!(27, Game::new(vec![1, 2, 3]).nth_number(2020).unwrap());
        assert_eq!(78, Game::new(vec![2, 3, 1]).nth_number(2020).unwrap());
        assert_eq!(438, Game::new(vec![3, 2, 1]).nth_number(2020).unwrap());
        assert_eq!(1836, Game::new(vec![3, 1, 2]).nth_number(2020).unwrap());
    }
}
