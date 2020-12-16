use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{all_consuming, complete, map_res},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use std::ops::RangeInclusive;

named!(field_name<&str, &str>, is_not!(":"));

fn range_parser(i: &str) -> IResult<&str, RangeInclusive<i32>> {
    named!(
        number_parser<&str, i32>,
        map_res!(digit1, |c| i32::from_str_radix(c, 10))
    );
    let (input, (start, _, end)) = tuple((number_parser, char('-'), number_parser))(i)?;
    IResult::Ok((input, start..=end))
}

#[derive(Debug, PartialEq)]
enum Info {
    Field(String, RangeInclusive<i32>, RangeInclusive<i32>),
    Ticket(Vec<i32>),
}

fn field_parser(input: &str) -> IResult<&str, (String, RangeInclusive<i32>, RangeInclusive<i32>)> {
    let (input, name) = field_name(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, a) = range_parser(input)?;
    let (input, _) = tag(" or ")(input)?;
    let (input, b) = range_parser(input)?;

    IResult::Ok((input, (String::from(name), a, b)))
}

fn ticket_parser(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(char(','), map_res(digit1, |n| i32::from_str_radix(n, 10)))(input)
}

fn parse_input(input: &str) -> (Vec<Info>, Vec<Info>) {
    input
        .lines()
        .filter_map(|line| {
            alt((
                map_res(
                    complete(all_consuming(field_parser)),
                    |(name, a, b)| -> Result<Info, !> { Ok(Info::Field(name, a, b)) },
                ),
                map_res(
                    complete(all_consuming(ticket_parser)),
                    |fields| -> Result<Info, !> { Ok(Info::Ticket(fields)) },
                ),
            ))(line)
            .ok()
            .map(|res| res.1)
        })
        .partition(|info| match info {
            Info::Field(..) => true,
            Info::Ticket(..) => false,
        })
}

pub fn solve_p1(input: &str) -> i32 {
    let (fields, tickets) = parse_input(input);
    let ranges = fields
        .iter()
        .flat_map(|info| match info {
            Info::Field(_, a, b) => vec![a, b],
            Info::Ticket(..) => vec![],
        })
        .collect::<Vec<_>>();
    tickets
        .iter()
        .map(|info| match info {
            Info::Field(..) => 0,
            Info::Ticket(fields) => fields
                .iter()
                .map(|field_val| {
                    if ranges.iter().all(|range| !range.contains(field_val)) {
                        field_val
                    } else {
                        &0
                    }
                })
                .sum(),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_INPUT: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test]
    fn check_parse() {
        assert_eq!(range_parser("23-12367"), Ok(("", 23..=12367)));
        assert_eq!(
            field_parser("arrival platform: 38-456 or 480-968"),
            Ok(("", (String::from("arrival platform"), 38..=456, 480..=968)))
        );
        assert_eq!(ticket_parser("38,6,12"), Ok(("", vec![38, 6, 12])));
        assert_eq!(
            parse_input(SAMPLE_INPUT),
            ((
                vec![
                    Info::Field(String::from("class"), 1..=3, 5..=7),
                    Info::Field(String::from("row"), 6..=11, 33..=44),
                    Info::Field(String::from("seat"), 13..=40, 45..=50)
                ],
                vec![
                    Info::Ticket(vec![7, 1, 14]),
                    Info::Ticket(vec![7, 3, 47]),
                    Info::Ticket(vec![40, 4, 50]),
                    Info::Ticket(vec![55, 2, 20]),
                    Info::Ticket(vec![38, 6, 12]),
                ]
            ))
        );
    }

    #[test]
    fn check_p1() {
        assert_eq!(solve_p1(SAMPLE_INPUT), 71);
        let input =
            std::fs::read_to_string("src/day16/input.in").expect("failed to read day16 input");
        assert_eq!(solve_p1(&input), 23009);
    }
}
