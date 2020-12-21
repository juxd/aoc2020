#![feature(drain_filter, never_type)]
#[macro_use]
extern crate nom;

pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day18;

#[cfg(test)]
mod tests {
    #[test]
    fn day_10_soln() {
        use crate::day10::*;

        let input =
            std::fs::read_to_string("src/day10/input.in").expect("failed to read day10 input");
        let sorted = sorted_input(&input);
        let (ones, threes) = get_ones_and_threes(&sorted);
        assert_eq!(ones * threes, 2380);
        assert_eq!(48358655787008, solve_part_two(&sorted));
    }

    #[test]
    fn day11_soln() {
        use super::day11::*;

        let input =
            std::fs::read_to_string("src/day11/input.in").expect("failed to read day11 input");
        assert_eq!(2261, solve_p1(&input));
        assert_eq!(2039, solve_p2(&input));
    }

    #[test]
    fn day12_soln() {
        use super::day12::*;

        let input =
            std::fs::read_to_string("src/day12/input.in").expect("failed to read day12 input");
        assert_eq!(1032, solve_p1(&input));
        assert_eq!(156735, solve_p2(&input));
    }

    #[test]
    fn day13_soln() {
        use super::day13::*;

        let input =
            std::fs::read_to_string("src/day13/input.in").expect("failed to read day13 input");
        assert_eq!(5946, solve_p1(&input));
        assert_eq!(645338524823718, solve_p2(&input));
    }

    #[test]
    fn day14_soln() {
        use super::day14::*;

        let input =
            std::fs::read_to_string("src/day14/input.in").expect("failed to read day14 input");
        assert_eq!(9628746976360, solve_p1(&input));
        assert_eq!(4574598714592, solve_p2(&input));
    }

    #[test]
    fn day15_soln() {
        use super::day15::*;

        assert_eq!(1280, solve_p1(&mut vec![2, 15, 0, 9, 1, 20]));
        assert_eq!(651639, solve_p2(&mut vec![2, 15, 0, 9, 1, 20]));
    }
}
