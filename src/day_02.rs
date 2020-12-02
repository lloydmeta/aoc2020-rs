use combine::parser::char::*;
use combine::*;
use std::error::Error as StdErr;

const DAY_2_INPUT: &str = include_str!("../data/day_02_input");

pub fn run() -> Result<(), Box<dyn StdErr>> {
    println!("*** Day 2: Password Philosophy ***");
    println!("Input: {}", DAY_2_INPUT);
    let policies_with_passwords = parse_input(DAY_2_INPUT)?;
    let valid_passwords_1 = find_valid_passwords_1(&policies_with_passwords);
    println!("Number of valid passwords 1: {}", valid_passwords_1.len());
    let valid_passwords_2 = find_valid_passwords_2(&policies_with_passwords);
    println!("Number of valid passwords 2: {}", valid_passwords_2.len());
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct PasswordPolicy {
    i: usize,
    j: usize,
    letter: char,
}

#[derive(Debug, PartialEq, Eq)]
struct Password(String);

#[derive(Debug, PartialEq, Eq)]
struct PasswordPolicyWithPassword {
    policy: PasswordPolicy,
    password: Password,
}

fn find_valid_passwords_1(v: &[PasswordPolicyWithPassword]) -> Vec<&Password> {
    v.iter()
        .filter_map(
            |&PasswordPolicyWithPassword {
                 ref policy,
                 ref password,
             }| {
                let occurrences_of_password_target = password
                    .0
                    .chars()
                    .into_iter()
                    .filter(|c| *c == policy.letter)
                    .count();
                if policy.i <= occurrences_of_password_target
                    && occurrences_of_password_target <= policy.j
                {
                    Some(password)
                } else {
                    None
                }
            },
        )
        .collect()
}

fn find_valid_passwords_2(v: &[PasswordPolicyWithPassword]) -> Vec<&Password> {
    v.iter()
        .filter_map(
            |&PasswordPolicyWithPassword {
                 ref policy,
                 ref password,
             }| {
                let password_chars: Vec<_> = password.0.chars().collect();
                let has_target_at_idx_1 = password_chars
                    .get(policy.i - 1) // idx starts at 1
                    .filter(|c| *c == &policy.letter)
                    .is_some();
                let has_target_at_idx_2 = password_chars
                    .get(policy.j - 1)
                    .filter(|c| *c == &policy.letter)
                    .is_some();
                if has_target_at_idx_1 ^ has_target_at_idx_2 {
                    Some(password)
                } else {
                    None
                }
            },
        )
        .collect()
}

fn parse_input(s: &str) -> Result<Vec<PasswordPolicyWithPassword>, Box<dyn StdErr + '_>> {
    let policy_parser = many::<String, _, _>(digit())
        .skip(char('-'))
        .and(many::<String, _, _>(digit()))
        .skip(space())
        .and(letter())
        .map(|((lower_bound_s, upper_bound_s), target)| {
            let i = lower_bound_s.parse().expect("number");
            let j = upper_bound_s.parse().expect("number");
            PasswordPolicy {
                i,
                j,
                letter: target,
            }
        });

    let policy_with_password_parser = policy_parser
        .skip(char(':'))
        .skip(spaces())
        .and(many::<String, _, _>(letter()))
        .map(|(policy, password)| PasswordPolicyWithPassword {
            policy,
            password: Password(password),
        });
    let mut parser = sep_by(policy_with_password_parser, spaces());
    let (r, _) = parser.easy_parse(s.trim())?;
    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_test() {
        let input = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
";
        let r = parse_input(input).unwrap();
        let expected = vec![
            PasswordPolicyWithPassword {
                policy: PasswordPolicy {
                    i: 1,
                    j: 3,
                    letter: 'a',
                },
                password: Password("abcde".to_string()),
            },
            PasswordPolicyWithPassword {
                policy: PasswordPolicy {
                    i: 1,
                    j: 3,
                    letter: 'b',
                },
                password: Password("cdefg".to_string()),
            },
            PasswordPolicyWithPassword {
                policy: PasswordPolicy {
                    i: 2,
                    j: 9,
                    letter: 'c',
                },
                password: Password("ccccccccc".to_string()),
            },
        ];
        assert_eq!(expected, r);
    }

    #[test]
    fn find_valid_passwords_1_test() {
        let policies_with_passwords = vec![
            PasswordPolicyWithPassword {
                policy: PasswordPolicy {
                    i: 1,
                    j: 3,
                    letter: 'a',
                },
                password: Password("abcde".to_string()),
            },
            PasswordPolicyWithPassword {
                policy: PasswordPolicy {
                    i: 1,
                    j: 3,
                    letter: 'b',
                },
                password: Password("cdefg".to_string()),
            },
            PasswordPolicyWithPassword {
                policy: PasswordPolicy {
                    i: 2,
                    j: 9,
                    letter: 'c',
                },
                password: Password("ccccccccc".to_string()),
            },
        ];
        let r = find_valid_passwords_1(&policies_with_passwords);
        let expected_values = vec![
            Password("abcde".to_string()),
            Password("ccccccccc".to_string()),
        ];
        let expected: Vec<_> = expected_values.iter().map(|p| p).collect();
        assert_eq!(expected, r)
    }

    #[test]
    fn find_valid_passwords_2_test() {
        let policies_with_passwords = vec![
            PasswordPolicyWithPassword {
                policy: PasswordPolicy {
                    i: 1,
                    j: 3,
                    letter: 'a',
                },
                password: Password("abcde".to_string()),
            },
            PasswordPolicyWithPassword {
                policy: PasswordPolicy {
                    i: 1,
                    j: 3,
                    letter: 'b',
                },
                password: Password("cdefg".to_string()),
            },
            PasswordPolicyWithPassword {
                policy: PasswordPolicy {
                    i: 2,
                    j: 9,
                    letter: 'c',
                },
                password: Password("ccccccccc".to_string()),
            },
        ];
        let r = find_valid_passwords_2(&policies_with_passwords);
        let expected_values = vec![Password("abcde".to_string())];
        let expected: Vec<_> = expected_values.iter().map(|p| p).collect();
        assert_eq!(expected, r)
    }
}
