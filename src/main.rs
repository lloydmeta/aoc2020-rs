extern crate aoc_2020;
extern crate clap;

use std::fmt::Display;
use std::str::FromStr;

use clap::{App, Arg, ArgMatches};
use std::error::Error;

use aoc_2020::day_01;
use aoc_2020::day_02;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Advent of Code 2017")
        .version(version().as_str())
        .about("Solutions to AoC 2017 !")
        .arg(
            Arg::with_name("day")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("Which day's solution you want to run"),
        )
        .get_matches();
    match get_number("day", Some(0), &matches) {
        1 => day_01::run()?,
        2 => day_02::run()?,

        other => return Err(format!("Invalid day: {}", other).into()),
    }
    Ok(())
}

fn version() -> String {
    let (maj, min, pat) = (
        option_env!("CARGO_PKG_VERSION_MAJOR"),
        option_env!("CARGO_PKG_VERSION_MINOR"),
        option_env!("CARGO_PKG_VERSION_PATCH"),
    );
    match (maj, min, pat) {
        (Some(maj), Some(min), Some(pat)) => format!("{}.{}.{}", maj, min, pat),
        _ => "".to_owned(),
    }
}

fn get_number<'a, A>(name: &str, maybe_min: Option<A>, matches: &ArgMatches<'a>) -> A
where
    A: FromStr + PartialOrd + Display + Copy,
    <A as FromStr>::Err: std::fmt::Debug,
{
    matches
        .value_of(name)
        .and_then(|s| s.parse::<A>().ok())
        .and_then(|u| match maybe_min {
            Some(min) => {
                if u > min {
                    Some(u)
                } else {
                    None
                }
            }
            _ => Some(u),
        })
        .expect(
            &{
                if let Some(min) = maybe_min {
                    format!("{} should be a positive number greater than {}.", name, min)
                } else {
                    format!("{} should be a positive number.", name)
                }
            }[..],
        )
}
