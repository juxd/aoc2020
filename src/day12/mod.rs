use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
enum Move {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32),
}

#[derive(Debug, PartialEq, Eq)]
struct Ship {
    dir: i32,
    x: i32,
    y: i32,
    waypoint_x: i32,
    waypoint_y: i32,
}

impl Move {
    fn of_dir_and_value(dir: &str, val: i32) -> Move {
        match dir {
            "N" => Move::N(val),
            "S" => Move::S(val),
            "E" => Move::E(val),
            "W" => Move::W(val),
            "L" => Move::L(val),
            "R" => Move::R(val),
            "F" => Move::F(val),
            _ => panic!("unknown dir"),
        }
    }
}

fn parse_input(input: &str) -> Vec<Move> {
    Regex::new(r"([NSEWLRF])(\d*)")
        .unwrap()
        .captures_iter(input)
        .map(|cap| Move::of_dir_and_value(&cap[1], str::parse::<i32>(&cap[2]).unwrap()))
        .collect::<Vec<_>>()
}

fn sim_moves(ship: &mut Ship, moves: &[Move]) {
    for mov in moves {
        match mov {
            Move::N(amt) => ship.y += amt,
            Move::S(amt) => ship.y -= amt,
            Move::E(amt) => ship.x += amt,
            Move::W(amt) => ship.x -= amt,
            Move::L(amt) => ship.dir = (ship.dir - amt + 360) % 360,
            Move::R(amt) => ship.dir = (ship.dir + amt + 360) % 360,
            Move::F(amt) => match ship.dir {
                0 => ship.y += amt,
                90 => ship.x += amt,
                180 => ship.y -= amt,
                270 => ship.x -= amt,
                other => panic!("lol, no non-90 value {} found", other),
            },
        }
    }
}

fn sim_moves_p2(ship: &mut Ship, moves: &[Move]) {
    for mov in moves {
        match mov {
            Move::N(amt) => ship.waypoint_y += amt,
            Move::S(amt) => ship.waypoint_y -= amt,
            Move::E(amt) => ship.waypoint_x += amt,
            Move::W(amt) => ship.waypoint_x -= amt,
            Move::L(amt) => {
                let mut turns = *amt;
                while turns > 0 {
                    let tmp = ship.waypoint_x;
                    ship.waypoint_x = -ship.waypoint_y;
                    ship.waypoint_y = tmp;
                    turns -= 90;
                }
            }
            Move::R(amt) => {
                let mut turns = *amt;
                while turns > 0 {
                    let tmp = ship.waypoint_x;
                    ship.waypoint_x = ship.waypoint_y;
                    ship.waypoint_y = -tmp;
                    turns -= 90;
                }
            }
            Move::F(amt) => {
                ship.x += ship.waypoint_x * amt;
                ship.y += ship.waypoint_y * amt;
            }
        }
    }
}

pub fn solve_p1(input: &str) -> i32 {
    let ship = &mut Ship {
        dir: 90,
        x: 0,
        y: 0,
        waypoint_x: 10,
        waypoint_y: 1,
    };
    let moves = parse_input(input);
    sim_moves(ship, &moves);
    ship.x.abs() + ship.y.abs()
}

pub fn solve_p2(input: &str) -> i32 {
    let ship = &mut Ship {
        dir: 90,
        x: 0,
        y: 0,
        waypoint_x: 10,
        waypoint_y: 1,
    };
    let moves = parse_input(input);
    sim_moves_p2(ship, &moves);
    ship.x.abs() + ship.y.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_INPUT: &str = "
F10
N3
F7
R90
F11";

    #[test]
    fn check_input_parse() {
        use Move::*;
        assert_eq!(
            vec![F(10), N(3), F(7), R(90), F(11)],
            parse_input(SAMPLE_INPUT)
        );
    }

    #[test]
    fn test_p1() {
        assert_eq!(25, solve_p1(SAMPLE_INPUT));
    }

    #[test]
    fn test_p2() {
        assert_eq!(286, solve_p2(SAMPLE_INPUT));
    }
}
