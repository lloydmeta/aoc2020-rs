use itertools::*;

const DAY_1_INPUT: &str = include_str!("../data/day_1_input");

const TARGET: isize = 2020;

pub fn run() -> Result<(), &'static str> {
    println!("*** Day 1: Report Repair ***");
    println!("Input: {}", DAY_1_INPUT);
    let nums = string_to_digits(DAY_1_INPUT);
    let sum_match_2020_2 = sum_match(&nums, 2, TARGET);
    let products_of_match_2020_2 = sum_match_2020_2.as_ref().map(|v| product_of_vec(v));
    println!(
        "Matches {:?} Solution 1: {:?}\n",
        sum_match_2020_2, products_of_match_2020_2
    );
    let sum_match_2020_3 = sum_match(&nums, 3, TARGET);
    let products_of_match_2020_3 = sum_match_2020_3.as_ref().map(|v| product_of_vec(v));
    println!(
        "Matches {:?} Solution 2: {:?}\n",
        sum_match_2020_3, products_of_match_2020_3
    );
    Ok(())
}

fn string_to_digits(s: &str) -> Vec<isize> {
    s.split("\n").filter_map(|v| v.parse().ok()).collect()
}

fn sum_match(nums: &Vec<isize>, element_count: usize, target: isize) -> Option<Vec<&isize>> {
    nums.iter()
        .combinations(element_count)
        .filter_map(|i| {
            let maybe_sum_of_vec = if i.is_empty() {
                None
            } else {
                Some(i.iter().fold(0, |acc, next| acc + **next))
            };
            let maybe_vec_where_elements_add_to_target: Option<Vec<&isize>> =
                maybe_sum_of_vec.and_then(|product| if product == target { Some(i) } else { None });
            maybe_vec_where_elements_add_to_target
        })
        .next()
}

fn product_of_vec(v: &Vec<&isize>) -> isize {
    v.iter().fold(1isize, |acc, next| acc * *next)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_to_digits_test() {
        assert_eq!(string_to_digits("1234\n5432"), vec![1234, 5432]);
    }

    #[test]
    fn sum_matches_2_test() {
        let vec = vec![1721, 979, 366, 299, 675, 1456];
        let expected = Some(vec![&1721, &299]);
        assert_eq!(expected, sum_match(&vec, 2, TARGET));
    }

    #[test]
    fn sum_matches_3_test() {
        let vec = vec![1721, 979, 366, 299, 675, 1456];
        let expected = Some(vec![&979, &366, &675]);
        assert_eq!(expected, sum_match(&vec, 3, TARGET));
    }
}
