use anyhow::Result;
use combine::easy::ParseError;
use combine::parser::char::*;
use combine::*;

use std::convert::TryInto;

use std::result::Result as StdResult;

const INPUT: &str = include_str!("../data/day_05_input");

pub fn run() -> Result<()> {
    println!("*** Day 5: Binary Boarding ***");
    println!("Input: {}", INPUT);
    let map = parse(INPUT)?;

    let max_id = map.iter().map(|seat| seat.id()).max();

    println!("Solution 1: {:?}", max_id);
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum RowPartition {
    Front,
    Back,
}

#[derive(Debug, PartialEq, Eq)]
enum ColumnPartition {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
struct Seat {
    row_partitions: [RowPartition; 7],
    column_partitions: [ColumnPartition; 3],
}

impl Seat {
    fn id(&self) -> usize {
        self.row() * 8 + self.column()
    }

    fn row(&self) -> usize {
        let (_, second) = self
            .row_partitions
            .iter()
            .fold((0, 127), |(first, last), next| {
                println!("next {:?}", next);
                let half = first + (((last as f32 - first as f32) / 2f32).floor() as usize);
                if next == &RowPartition::Front {
                    println!("{:?}", (first, half));
                    (first, half)
                } else {
                    println!("{:?}", (half, last));
                    (half, last)
                }
            });
        second
    }

    fn column(&self) -> usize {
        let (_, second) = self
            .column_partitions
            .iter()
            .fold((0, 7), |(first, last), next| {
                println!("next {:?}", next);
                let half = first + (((last as f32 - first as f32) / 2f32).floor() as usize);
                println!("half {:?}", half);
                if next == &ColumnPartition::Left {
                    println!("{:?}", (first, half));
                    (first, half)
                } else {
                    println!("{:?}", (half, last));
                    (half, last)
                }
            });
        second
    }
}

fn parse(s: &str) -> StdResult<Vec<Seat>, ParseError<&str>> {
    let row_partition_parser = char('B')
        .map(|_| RowPartition::Back)
        .or(char('F').map(|_| RowPartition::Front));
    let column_partition_parser = char('L')
        .map(|_| ColumnPartition::Left)
        .or(char('R').map(|_| ColumnPartition::Right));
    let row_parser = count_min_max(7, 7, row_partition_parser)
        .and(count_min_max(3, 3, column_partition_parser))
        .map(
            |(row_partitions, column_partitions): (Vec<RowPartition>, Vec<ColumnPartition>)| Seat {
                row_partitions: row_partitions
                    .try_into()
                    .expect("Count should match, it's in the parser..."),
                column_partitions: column_partitions
                    .try_into()
                    .expect("Count should match, it's in the parser..."),
            },
        );
    let mut map_parser = many(row_parser.skip(spaces()));
    let (r, _) = map_parser.easy_parse(s)?;
    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ColumnPartition::*;
    use RowPartition::*;

    #[test]
    fn parse_test() {
        let input = "BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL
";
        let r = parse(&input).unwrap();
        let expected = vec![
            Seat {
                row_partitions: [Back, Front, Front, Front, Back, Back, Front],
                column_partitions: [Right, Right, Right],
            },
            Seat {
                row_partitions: [Front, Front, Front, Back, Back, Back, Front],
                column_partitions: [Right, Right, Right],
            },
            Seat {
                row_partitions: [Back, Back, Front, Front, Back, Back, Front],
                column_partitions: [Right, Left, Left],
            },
        ];
        assert_eq!(expected, r);
    }

    #[test]
    fn entry_row_test() {
        let entry_1 = Seat {
            row_partitions: [Front, Back, Front, Back, Back, Front, Front],
            column_partitions: [Right, Left, Right],
        };
        let entry_2 = Seat {
            row_partitions: [Back, Front, Front, Front, Back, Back, Front],
            column_partitions: [Right, Right, Right],
        };
        let entry_3 = Seat {
            row_partitions: [Front, Front, Front, Back, Back, Back, Front],
            column_partitions: [Right, Right, Right],
        };
        let entry_4 = Seat {
            row_partitions: [Back, Back, Front, Front, Back, Back, Front],
            column_partitions: [Right, Left, Left],
        };
        assert_eq!(44, entry_1.row());
        assert_eq!(70, entry_2.row());
        assert_eq!(14, entry_3.row());
        assert_eq!(102, entry_4.row());
    }

    #[test]
    fn entry_column_test() {
        let entry_1 = Seat {
            row_partitions: [Front, Back, Front, Back, Back, Front, Front],
            column_partitions: [Right, Left, Right],
        };
        let entry_2 = Seat {
            row_partitions: [Back, Front, Front, Front, Back, Back, Front],
            column_partitions: [Right, Right, Right],
        };
        let entry_3 = Seat {
            row_partitions: [Front, Front, Front, Back, Back, Back, Front],
            column_partitions: [Right, Right, Right],
        };
        let entry_4 = Seat {
            row_partitions: [Back, Back, Front, Front, Back, Back, Front],
            column_partitions: [Right, Left, Left],
        };
        assert_eq!(5, entry_1.column());
        assert_eq!(7, entry_2.column());
        assert_eq!(7, entry_3.column());
        assert_eq!(4, entry_4.column());
    }

    #[test]
    fn entry_id_test() {
        let entry_1 = Seat {
            row_partitions: [Back, Front, Front, Front, Back, Back, Front],
            column_partitions: [Right, Right, Right],
        };
        let entry_2 = Seat {
            row_partitions: [Front, Front, Front, Back, Back, Back, Front],
            column_partitions: [Right, Right, Right],
        };
        let entry_3 = Seat {
            row_partitions: [Back, Back, Front, Front, Back, Back, Front],
            column_partitions: [Right, Left, Left],
        };
        assert_eq!(567, entry_1.id());
        assert_eq!(119, entry_2.id());
        assert_eq!(820, entry_3.id());
    }
}
