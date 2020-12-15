pub fn sorted_input(input: &str) -> Vec<i32> {
    let mut parsed_input = input
        .split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    parsed_input.sort_unstable();
    parsed_input
}

pub fn get_ones_and_threes(sorted_numbers: &[i32]) -> (i32, i32) {
    let mut prev_number = 0;
    let mut ones = 0;
    let mut threes = 1;
    for next_number in sorted_numbers.iter() {
        match *next_number - prev_number {
            1 => ones += 1,
            3 => threes += 1,
            _ => (),
        }
        prev_number = *next_number;
    }
    (ones, threes)
}

pub fn solve_part_two(sorted_numbers: &[i32]) -> i64 {
    let mut num_of_ways = vec![0; sorted_numbers.len()];
    num_of_ways[sorted_numbers.len() - 1] = 1;
    for i in (0..sorted_numbers.len() - 1).rev() {
        let mut j = i + 1;
        while j < sorted_numbers.len() && sorted_numbers[j] - sorted_numbers[i] <= 3 {
            num_of_ways[i] += num_of_ways[j];
            j += 1;
        }
    }
    (0..3)
        .map(|i| {
            sorted_numbers
                .get(i)
                .map(|v| if *v <= 3 { num_of_ways[i] } else { 0 })
                .unwrap_or(0)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day10::*;
    static SAMPLE_INPUT: &str = "
16
10
15
5
1
11
7
19
6
12
4
";

    static SAMPLE_INPUT2: &str = "
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
";

    #[test]
    fn sort_sample_input() {
        assert_eq!(
            vec![1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19],
            sorted_input(&SAMPLE_INPUT)
        );
    }

    #[test]
    fn count_ones_and_threes() {
        assert_eq!((7, 5), get_ones_and_threes(&sorted_input(SAMPLE_INPUT)));
    }

    #[test]
    fn part_two() {
        assert_eq!(8, solve_part_two(&sorted_input(SAMPLE_INPUT)));
        assert_eq!(19208, solve_part_two(&sorted_input(SAMPLE_INPUT2)));
    }
}
