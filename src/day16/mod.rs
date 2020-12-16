use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{all_consuming, complete, map_res},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use std::collections::{HashMap, HashSet, VecDeque};
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
    let (input, (name, _, a, _, b)) = tuple((
        field_name,
        tag(": "),
        range_parser,
        tag(" or "),
        range_parser,
    ))(input)?;

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

fn all_ranges(fields: &[Info]) -> Vec<&RangeInclusive<i32>> {
    fields
        .iter()
        .flat_map(|info| match info {
            Info::Field(_, a, b) => vec![a, b],
            Info::Ticket(..) => vec![],
        })
        .collect::<Vec<_>>()
}

pub fn solve_p1(input: &str) -> i32 {
    let (fields, tickets) = parse_input(input);
    let ranges = all_ranges(&fields);
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

pub fn solve_p2(input: &str) -> HashMap<String, Vec<usize>> {
    let (fields, tickets) = parse_input(input);
    let fields_as_tuples = fields
        .iter()
        .filter_map(|info| match info {
            Info::Field(name, a, b) => Some((name, a, b)),
            Info::Ticket(..) => None,
        })
        .collect::<Vec<_>>();
    let all_ranges = all_ranges(&fields);
    let valid_tickets = tickets[1..]
        .iter()
        .filter_map(|info| match info {
            Info::Field(..) => None,
            Info::Ticket(fields) => {
                if fields
                    .iter()
                    .any(|field_val| all_ranges.iter().all(|range| !range.contains(field_val)))
                {
                    None
                } else {
                    Some(fields)
                }
            }
        })
        .collect::<Vec<_>>();
    let candidates_per_field = valid_tickets
        .iter()
        .fold(
            vec![fields_as_tuples; valid_tickets[0].len()],
            |mut candidates, ticket| {
                candidates
                    .iter_mut()
                    .zip(ticket.iter())
                    .map(|(previous_candidates, field_val)| {
                        previous_candidates.drain_filter(|(_, a, b)| {
                            !a.contains(field_val) && !b.contains(field_val)
                        });
                        previous_candidates.to_vec()
                    })
                    .collect::<Vec<_>>()
            },
        )
        .iter()
        .map(|c| {
            c.iter()
                .map(|(name, ..)| String::from(*name))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut candidate_field_appearances = HashMap::new();
    let mut seen = HashSet::new();
    for (field_index, candidates) in candidates_per_field.iter().enumerate() {
        for candidate in candidates {
            candidate_field_appearances
                .entry(candidate.to_string())
                .or_insert_with(Vec::new)
                .push(field_index);
        }
    }
    let mut pop_q = VecDeque::new();
    for (candidate, indices) in candidate_field_appearances.iter() {
        if indices.len() == 1 {
            pop_q.push_back((candidate.to_string(), indices[0]));
            seen.insert(candidate.to_string());
        }
    }
    while let Some((candidate, index)) = pop_q.pop_front() {
        for (candidate_o, indices) in candidate_field_appearances.iter_mut() {
            if candidate_o == &candidate {
                continue;
            }
            indices.drain_filter(|idx| idx == &index);
            if indices.len() == 1 && !seen.contains(candidate_o) {
                pop_q.push_back((candidate_o.to_string(), indices[0]));
                seen.insert(candidate_o.to_string());
            }
        }
    }
    candidate_field_appearances
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

    static SAMPLE_INPUT2: &str = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

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

    #[test]
    fn check_p2() {
        let input =
            std::fs::read_to_string("src/day16/input.in").expect("failed to read day16 input");
        assert_eq!(
            solve_p2(SAMPLE_INPUT2),
            vec![
                ("class".to_string(), vec![1usize]),
                ("row".to_string(), vec![0]),
                ("seat".to_string(), vec![2])
            ]
            .into_iter()
            .collect()
        );
        assert_eq!(
            solve_p2(&input),
            vec![
                ("row".to_string(), vec![0]),
                ("seat".to_string(), vec![1]),
                ("arrival location".to_string(), vec![2]),
                ("duration".to_string(), vec![3]),
                ("departure date".to_string(), vec![4]),
                ("route".to_string(), vec![5]),
                ("wagon".to_string(), vec![6]),
                ("departure station".to_string(), vec![7]),
                ("price".to_string(), vec![8]),
                ("departure location".to_string(), vec![9]),
                ("departure platform".to_string(), vec![10]),
                ("departure track".to_string(), vec![11]),
                ("zone".to_string(), vec![12]),
                ("type".to_string(), vec![13]),
                ("departure time".to_string(), vec![14]),
                ("arrival track".to_string(), vec![15]),
                ("arrival station".to_string(), vec![16]),
                ("class".to_string(), vec![17]),
                ("arrival platform".to_string(), vec![18]),
                ("train".to_string(), vec![19]),
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn ans_p2() {
        let mine = ticket_parser(
            "79,193,53,97,137,179,131,73,191,139,197,181,67,71,211,199,167,61,59,127",
        )
        .ok()
        .unwrap()
        .1
        .iter()
        .map(|x| *x as i64)
        .collect::<Vec<_>>();
        assert_eq!(
            mine[4] * mine[7] * mine[9] * mine[10] * mine[11] * mine[14],
            1
        );
    }
}
