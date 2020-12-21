use nom::{
    branch::alt,
    character::complete::{char, digit1, multispace0, one_of},
    combinator::{map, map_res},
    multi::fold_many0,
    sequence::tuple,
    IResult,
};

use std::collections::VecDeque;

#[derive(Clone, Debug, PartialEq)]
enum Term {
    Imm(i32),
    Stm(Box<Term>, Vec<(char, Term)>),
}

fn parse_tail(stmt: &str) -> IResult<&str, Vec<(char, Term)>> {
    fold_many0(
        tuple((multispace0, one_of("+*"), multispace0, parse_term)),
        Vec::new(),
        |mut acc: Vec<_>, (_, op, _, term): (&str, char, &str, Term)| {
            acc.push((op, term));
            acc
        },
    )(stmt)
}

fn parse_num(stmt: &str) -> IResult<&str, i32> {
    map_res(digit1, |c| i32::from_str_radix(c, 10))(stmt)
}

fn parse_statement(stmt: &str) -> IResult<&str, Term> {
    let (input, (hd, tl)) = tuple((parse_term, parse_tail))(stmt)?;
    IResult::Ok((input, Term::Stm(Box::new(hd), tl)))
}

fn parse_term(term: &str) -> IResult<&str, Term> {
    alt((
        map(parse_num, Term::Imm),
        map(
            tuple((char('('), parse_statement, char(')'))),
            |(_, stm, _)| stm,
        ),
    ))(term)
}

fn eval_term(term: Term) -> i64 {
    match term {
        Term::Imm(v) => v as i64,
        Term::Stm(init, rest) => {
            let mut v = eval_term(*init) as i64;
            for (op, term) in rest {
                match op {
                    '+' => v += eval_term(term),
                    '*' => v *= eval_term(term),
                    _ => panic!("unexpected op"),
                }
            }
            v
        }
    }
}

fn eval_term_2(term: Term) -> i64 {
    match term {
        Term::Imm(v) => v as i64,
        Term::Stm(init, rest) => {
            let mut op_stk = VecDeque::new();
            let mut num_stk = VecDeque::new();
            num_stk.push_back(eval_term_2(*init));
            for (op, term) in rest {
                match op {
                    '+' => match op_stk.back() {
                        None | Some('*') => (),
                        Some('+') => {
                            let a = num_stk.pop_back().unwrap();
                            let b = num_stk.pop_back().unwrap();
                            op_stk.pop_back();
                            num_stk.push_back(a + b);
                        }
                        _ => panic!("unexpected pattern"),
                    },
                    '*' => loop {
                        match op_stk.back() {
                            None => break,
                            Some(op) => {
                                let a = num_stk.pop_back().unwrap();
                                let b = num_stk.pop_back().unwrap();
                                let res = match op {
                                    '+' => a + b,
                                    '*' => a * b,
                                    _ => panic!("unknown op"),
                                };
                                op_stk.pop_back();
                                num_stk.push_back(res);
                            }
                        }
                    },
                    _ => panic!("unexpected op"),
                }
                num_stk.push_back(eval_term_2(term));
                op_stk.push_back(op)
            }
            while let Some(op) = op_stk.pop_back() {
                let a = num_stk.pop_back().unwrap();
                let b = num_stk.pop_back().unwrap();
                let res = match op {
                    '+' => a + b,
                    '*' => a * b,
                    _ => panic!("unexpected op"),
                };
                num_stk.push_back(res);
            }
            num_stk.pop_back().unwrap()
        }
    }
}

pub fn solve_p1() -> i64 {
    std::fs::read_to_string("src/day18/input.in")
        .expect("failed to read day18 input")
        .lines()
        .map(|stm| eval_term(parse_statement(stm).unwrap().1))
        .sum()
}

pub fn solve_p2() -> i64 {
    std::fs::read_to_string("src/day18/input.in")
        .expect("failed to read day18 input")
        .lines()
        .map(|stm| eval_term_2(parse_statement(stm).unwrap().1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        assert_eq!(parse_term("10"), IResult::Ok(("", Term::Imm(10))));
        assert_eq!(
            parse_tail(" + 10"),
            IResult::Ok(("", vec![('+', Term::Imm(10))]))
        );
        assert_eq!(
            parse_statement("1 + 10 * 3"),
            IResult::Ok((
                "",
                Term::Stm(
                    Box::new(Term::Imm(1)),
                    vec![('+', Term::Imm(10)), ('*', Term::Imm(3))]
                )
            ))
        );

        assert_eq!(
            parse_statement("(1 + 2 * 3) + 10 * 3"),
            IResult::Ok((
                "",
                Term::Stm(
                    Box::new(Term::Stm(
                        Box::new(Term::Imm(1)),
                        vec![('+', Term::Imm(2)), ('*', Term::Imm(3))]
                    )),
                    vec![('+', Term::Imm(10)), ('*', Term::Imm(3))]
                )
            ))
        );
    }

    #[test]
    fn test_eval() {
        assert_eq!(
            eval_term(parse_statement("(1 + 2 * 3) + 10 * 3").unwrap().1),
            57
        );
    }

    #[test]
    fn test_p1() {
        assert_eq!(solve_p1(), 209335026987);
    }

    #[test]
    fn test_eval2() {
        assert_eq!(
            eval_term_2(parse_statement("1 + 2 * 3 + 4 * 5 + 6").unwrap().1),
            231
        );
        assert_eq!(
            eval_term_2(parse_statement("5 + (8 * 3 + 9 + 3 * 4 * 3)").unwrap().1),
            1445
        );
        assert_eq!(
            eval_term_2(parse_statement("2 * 3 + (4 * 5)").unwrap().1),
            46
        );
    }

    #[test]
    fn test_p2() {
        assert_eq!(solve_p2(), 33331817392479);
    }
}
