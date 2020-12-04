//! This was a rough one...the different parsing options basically
//! kill the Rust compiler unless all the different parsers are
//! put in their own function as opposed to local bindings (
//! Compile times went up to _minutes_)
//!
//! This implementation needs revisiting when the kid is sleeping..

use anyhow::Result;
use combine::parser::char::*;
use combine::*;
use std::num::ParseIntError;

const INPUT: &str = include_str!("../data/day_04_input");

pub fn run() -> Result<()> {
    println!("*** Day 4: Passport Processing ***");
    println!("Input: {}", INPUT);
    let data = parse_raw(INPUT);
    let valid_count = count_valid_1(&data);

    println!("Solution 1: {}", valid_count);

    let validated_data = parse_validated(INPUT);
    let validated_count = validated_data.len();
    println!("Solution 2: {}", validated_count);
    Ok(())
}

enum RawDataField {
    BirthYear(usize),
    IssueYear(usize),
    ExpirationYear(usize),
    Height(String),
    HairColour(String),
    EyeColour(String),
    PassportId(String),
    CountryId(usize),
}

#[derive(Debug, PartialEq, Eq)]
struct RawData {
    birth_year: Option<usize>,
    issue_year: Option<usize>,
    expiration_year: Option<usize>,
    height: Option<String>,
    hair_colour: Option<String>,
    eye_colour: Option<String>,
    passport_id: Option<String>,
    country_id: Option<usize>,
}

impl RawData {
    fn blank() -> RawData {
        RawData {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_colour: None,
            eye_colour: None,
            passport_id: None,
            country_id: None,
        }
    }

    fn is_valid_1(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_colour.is_some()
            && self.eye_colour.is_some()
            && self.passport_id.is_some()
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ValidHeight {
    Inches(usize),
    Cm(usize),
}

#[derive(Debug, PartialEq, Eq)]
enum EyeColour {
    Amb,
    Blu,
    Brn,
    Gry,
    Grn,
    Hzl,
    Oth,
}

#[derive(Debug, PartialEq, Eq)]
enum ValidatedDataField {
    BirthYear(usize),
    IssueYear(usize),
    ExpirationYear(usize),
    Height(ValidHeight),
    HairColour(String),
    EyeColour(EyeColour),
    PassportId(String),
    CountryId(String),
}

#[derive(Debug, PartialEq, Eq)]
struct ValidatedDataBuilder {
    birth_year: Option<usize>,
    issue_year: Option<usize>,
    expiration_year: Option<usize>,
    height: Option<ValidHeight>,
    hair_colour: Option<String>,
    eye_colour: Option<EyeColour>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl ValidatedDataBuilder {
    fn blank() -> ValidatedDataBuilder {
        ValidatedDataBuilder {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_colour: None,
            eye_colour: None,
            passport_id: None,
            country_id: None,
        }
    }

    fn build(self) -> Option<ValidatedData> {
        Some(ValidatedData {
            birth_year: self.birth_year?,
            issue_year: self.issue_year?,
            expiration_year: self.expiration_year?,
            height: self.height?,
            hair_colour: self.hair_colour?,
            eye_colour: self.eye_colour?,
            passport_id: self.passport_id?,
            country_id: self.country_id,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ValidatedData {
    birth_year: usize,
    issue_year: usize,
    expiration_year: usize,
    height: ValidHeight,
    hair_colour: String,
    eye_colour: EyeColour,
    passport_id: String,
    country_id: Option<String>,
}

fn count_valid_1(v: &[RawData]) -> usize {
    v.iter().filter(|d| d.is_valid_1()).count()
}

fn number_parser<Input>(label: &'static str) -> impl Parser<Input, Output = usize>
where
    Input: Stream<Token = char>,
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
    attempt(string(label))
        .with(many::<String, _, _>(digit()))
        .and_then(|s| s.parse::<usize>())
}

fn birth_year_parser<Input>() -> impl Parser<Input, Output = RawDataField>
where
    Input: Stream<Token = char>,
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
    number_parser("byr:").map(RawDataField::BirthYear)
}

fn issue_year_parser<Input>() -> impl Parser<Input, Output = RawDataField>
where
    Input: Stream<Token = char>,
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
    number_parser("iyr:").map(RawDataField::IssueYear)
}

fn expiration_year_parser<Input>() -> impl Parser<Input, Output = RawDataField>
where
    Input: Stream<Token = char>,
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
    number_parser("eyr:").map(RawDataField::ExpirationYear)
}

fn passport_id_parser<Input>() -> impl Parser<Input, Output = RawDataField>
where
    Input: Stream<Token = char>,
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
    attempt(string("pid:"))
        .with(many::<String, _, _>(alpha_num().or(char('#'))))
        .map(RawDataField::PassportId)
}

fn country_id_parser<Input>() -> impl Parser<Input, Output = RawDataField>
where
    Input: Stream<Token = char>,
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
    number_parser("cid:").map(RawDataField::CountryId)
}

fn height_parser<Input>() -> impl Parser<Input, Output = RawDataField>
where
    Input: Stream<Token = char>,
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
    attempt(string("hgt:"))
        .with(many::<String, _, _>(alpha_num()))
        .map(RawDataField::Height)
}

fn eye_colour_parser<Input>() -> impl Parser<Input, Output = RawDataField>
where
    Input: Stream<Token = char>,
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
    attempt(string("ecl:"))
        .with(many::<String, _, _>(alpha_num().or(char('#'))))
        .map(RawDataField::EyeColour)
}

fn hair_colour_parser<Input>() -> impl Parser<Input, Output = RawDataField>
where
    Input: Stream<Token = char>,
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
    attempt(string("hcl:"))
        .with(many::<String, _, _>(alpha_num().or(char('#'))).map(RawDataField::HairColour))
}

fn data_field_parser<Input>() -> impl Parser<Input, Output = RawDataField>
where
    Input: Stream<Token = char>,
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
    birth_year_parser()
        .or(issue_year_parser())
        .or(expiration_year_parser())
        .or(height_parser())
        .or(hair_colour_parser())
        .or(eye_colour_parser())
        .or(passport_id_parser())
        .or(country_id_parser())
}

fn validated_birth_year_parser<Input>() -> impl Parser<Input, Output = ValidatedDataField>
where
    Input: Stream<Token = char>,
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
    number_parser("byr:")
        .and_then(|i| {
            if 1920 <= i && i <= 2002 {
                Ok(i)
            } else {
                "invalid__".parse()
            }
        })
        .map(ValidatedDataField::BirthYear)
}

fn validated_issue_year_parser<Input>() -> impl Parser<Input, Output = ValidatedDataField>
where
    Input: Stream<Token = char>,
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
    number_parser("iyr:")
        .and_then(|i| {
            if 2010 <= i && i <= 2020 {
                Ok(i)
            } else {
                "invalid__".parse()
            }
        })
        .map(ValidatedDataField::IssueYear)
}

fn validated_expiration_year_parser<Input>() -> impl Parser<Input, Output = ValidatedDataField>
where
    Input: Stream<Token = char>,
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
    number_parser("eyr:")
        .and_then(|i| {
            if 2020 <= i && i <= 2030 {
                Ok(i)
            } else {
                "invalid__".parse()
            }
        })
        .map(ValidatedDataField::ExpirationYear)
}

fn validated_height_parser<Input>() -> impl Parser<Input, Output = ValidatedDataField>
where
    Input: Stream<Token = char>,
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
    attempt(string("hgt:"))
        .with(
            attempt(
                many::<String, _, _>(digit())
                    .skip(string("cm"))
                    .and_then(|s| {
                        s.parse().and_then(|i| {
                            if 150 <= i && i <= 193 {
                                Ok(i)
                            } else {
                                "invalid__".parse()
                            }
                        })
                    })
                    .map(ValidHeight::Cm),
            )
            .or(attempt(
                many::<String, _, _>(digit())
                    .skip(string("in"))
                    .and_then(|s| {
                        s.parse().and_then(|i| {
                            if 59 <= i && i <= 76 {
                                Ok(i)
                            } else {
                                "invalid__".parse()
                            }
                        })
                    })
                    .map(ValidHeight::Inches),
            )),
        )
        .map(ValidatedDataField::Height)
}

fn validated_hair_colour_parser<Input>() -> impl Parser<Input, Output = ValidatedDataField>
where
    Input: Stream<Token = char>,
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
    attempt(string("hcl:#"))
        .with(count::<String, _, _>(6, hex_digit()).map(ValidatedDataField::HairColour))
}

fn validated_eye_colour_parser<Input>() -> impl Parser<Input, Output = ValidatedDataField>
where
    Input: Stream<Token = char>,
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
    attempt(string("ecl:"))
        .with(
            attempt(string("amb"))
                .map(|_| EyeColour::Amb)
                .or(attempt(string("blu")).map(|_| EyeColour::Blu))
                .or(attempt(string("brn")).map(|_| EyeColour::Brn))
                .or(attempt(string("gry")).map(|_| EyeColour::Gry))
                .or(attempt(string("grn")).map(|_| EyeColour::Grn))
                .or(attempt(string("hzl")).map(|_| EyeColour::Hzl))
                .or(attempt(string("oth")).map(|_| EyeColour::Oth)),
        )
        .map(ValidatedDataField::EyeColour)
}

fn validated_passport_id_parser<Input>() -> impl Parser<Input, Output = ValidatedDataField>
where
    Input: Stream<Token = char>,
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
    attempt(string("pid:"))
        .with(count_min_max::<String, _, _>(9, 9, digit()))
        .map(ValidatedDataField::PassportId)
}

fn validated_country_id_parser<Input>() -> impl Parser<Input, Output = ValidatedDataField>
where
    Input: Stream<Token = char>,
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
    attempt(string("cid:"))
        .with(many::<String, _, _>(alpha_num()))
        .map(ValidatedDataField::CountryId)
}

fn validated_data_field_parser<Input>() -> impl Parser<Input, Output = ValidatedDataField>
where
    Input: Stream<Token = char>,
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
    validated_birth_year_parser()
        .or(validated_issue_year_parser())
        .or(validated_expiration_year_parser())
        .or(validated_height_parser())
        .or(validated_hair_colour_parser())
        .or(validated_eye_colour_parser())
        .or(validated_passport_id_parser())
        .or(validated_country_id_parser())
}

fn data_parser<Input>() -> impl Parser<Input, Output = RawData>
where
    Input: Stream<Token = char>,
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
    sep_by(data_field_parser(), space()).map(|fields: Vec<RawDataField>| {
        fields.into_iter().fold(RawData::blank(), |mut acc, field| {
            match field {
                RawDataField::BirthYear(y) => acc.birth_year = Some(y),
                RawDataField::IssueYear(y) => acc.issue_year = Some(y),
                RawDataField::ExpirationYear(y) => acc.expiration_year = Some(y),
                RawDataField::Height(s) => acc.height = Some(s),
                RawDataField::HairColour(s) => acc.hair_colour = Some(s),
                RawDataField::EyeColour(s) => acc.eye_colour = Some(s),
                RawDataField::PassportId(i) => acc.passport_id = Some(i),
                RawDataField::CountryId(i) => acc.country_id = Some(i),
            }
            acc
        })
    })
}

fn validated_data_parser<Input>() -> impl Parser<Input, Output = Option<ValidatedData>>
where
    Input: Stream<Token = char>,
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
    sep_by(validated_data_field_parser(), space()).map(|fields: Vec<ValidatedDataField>| {
        let builder = fields
            .into_iter()
            .fold(ValidatedDataBuilder::blank(), |mut acc, field| {
                match field {
                    ValidatedDataField::BirthYear(y) => acc.birth_year = Some(y),
                    ValidatedDataField::IssueYear(y) => acc.issue_year = Some(y),
                    ValidatedDataField::ExpirationYear(y) => acc.expiration_year = Some(y),
                    ValidatedDataField::Height(s) => acc.height = Some(s),
                    ValidatedDataField::HairColour(s) => acc.hair_colour = Some(s),
                    ValidatedDataField::EyeColour(s) => acc.eye_colour = Some(s),
                    ValidatedDataField::PassportId(i) => acc.passport_id = Some(i),
                    ValidatedDataField::CountryId(i) => acc.country_id = Some(i),
                }
                acc
            });
        builder.build()
    })
}

fn parse_raw(s: &str) -> Vec<RawData> {
    // Just give up on doing this purely with parsers... the double newline is screwing me up
    s.trim()
        .split("\n\n")
        .filter_map(|section| {
            let mut parser = data_parser();
            let (r, _) = parser.easy_parse(section).ok()?;
            Some(r)
        })
        .collect()
}

fn parse_validated(s: &str) -> Vec<ValidatedData> {
    // Just give up on doing this purely with parsers... the double newline is screwing me up
    s.trim()
        .split("\n\n")
        .filter_map(|section| {
            let mut parser = validated_data_parser();
            let (r, _) = parser.easy_parse(section).ok()?;
            r
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_parse_test() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm";
        let mut parser = data_parser();
        let (r, _) = parser.easy_parse(input).unwrap();
        let expected = RawData {
            birth_year: Some(1937),
            issue_year: Some(2017),
            expiration_year: Some(2020),
            height: Some("183cm".to_string()),
            hair_colour: Some("#fffffd".to_string()),
            eye_colour: Some("gry".to_string()),
            passport_id: Some(860033327.to_string()),
            country_id: Some(147),
        };
        assert_eq!(expected, r);
    }

    #[test]
    fn parse_test() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
";
        let r = parse_raw(input);
        let expected = vec![
            RawData {
                birth_year: Some(1937),
                issue_year: Some(2017),
                expiration_year: Some(2020),
                height: Some("183cm".to_string()),
                hair_colour: Some("#fffffd".to_string()),
                eye_colour: Some("gry".to_string()),
                passport_id: Some(860033327.to_string()),
                country_id: Some(147),
            },
            RawData {
                birth_year: Some(1929),
                issue_year: Some(2013),
                expiration_year: Some(2023),
                height: None,
                hair_colour: Some("#cfa07d".to_string()),
                eye_colour: Some("amb".to_string()),
                passport_id: Some("028048884".to_string()),
                country_id: Some(350),
            },
            RawData {
                birth_year: Some(1931),
                issue_year: Some(2013),
                expiration_year: Some(2024),
                height: Some("179cm".to_string()),
                hair_colour: Some("#ae17e1".to_string()),
                eye_colour: Some("brn".to_string()),
                passport_id: Some(760753108.to_string()),
                country_id: None,
            },
            RawData {
                birth_year: None,
                issue_year: Some(2011),
                expiration_year: Some(2025),
                height: Some("59in".to_string()),
                hair_colour: Some("#cfa07d".to_string()),
                eye_colour: Some("brn".to_string()),
                passport_id: Some(166559648.to_string()),
                country_id: None,
            },
        ];
        assert_eq!(expected, r);
    }

    #[test]
    fn count_valid_test() {
        let data = vec![
            RawData {
                birth_year: Some(1937),
                issue_year: Some(2017),
                expiration_year: Some(2020),
                height: Some("183cm".to_string()),
                hair_colour: Some("#fffffd".to_string()),
                eye_colour: Some("gry".to_string()),
                passport_id: Some(860033327.to_string()),
                country_id: Some(147),
            },
            RawData {
                birth_year: Some(1929),
                issue_year: Some(2013),
                expiration_year: Some(2023),
                height: None,
                hair_colour: Some("#cfa07d".to_string()),
                eye_colour: Some("amb".to_string()),
                passport_id: Some("028048884".to_string()),
                country_id: Some(350),
            },
            RawData {
                birth_year: Some(1931),
                issue_year: Some(2013),
                expiration_year: Some(2024),
                height: Some("179cm".to_string()),
                hair_colour: Some("#ae17e1".to_string()),
                eye_colour: Some("brn".to_string()),
                passport_id: Some(760753108.to_string()),
                country_id: None,
            },
            RawData {
                birth_year: None,
                issue_year: Some(2011),
                expiration_year: Some(2025),
                height: Some("59in".to_string()),
                hair_colour: Some("#cfa07d".to_string()),
                eye_colour: Some("brn".to_string()),
                passport_id: Some(166559648.to_string()),
                country_id: None,
            },
        ];
        assert_eq!(2, count_valid_1(&data));
    }

    #[test]
    fn real_input_count_test() {
        let data = parse_raw(INPUT);
        let valid_count = count_valid_1(&data);
        assert_eq!(256, valid_count);
    }

    #[test]
    fn invalid_validated_input_parse_test() {
        let input = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        let validated_data = parse_validated(input);
        assert_eq!(0, validated_data.len())
    }

    #[test]
    fn valid_validated_input_parse_test() {
        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let validated_data = parse_validated(input);
        assert_eq!(4, validated_data.len())
    }

    #[test]
    fn second_task_test() {
        let validated_data = parse_validated(INPUT);
        let validated_count = validated_data.len();
        assert_eq!(198, validated_count);
    }
}
