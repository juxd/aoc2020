use std::collections::HashMap;

fn solve(input: &mut Vec<u32>, turns: u32) -> u32 {
    let mut seen = HashMap::new();
    let mut last = input.pop().unwrap();
    for (i, n) in input.iter().enumerate() {
        seen.insert(*n, i as u32 + 1);
    }
    for i in input.len() + 2..=(turns as usize) {
        if seen.contains_key(&last) {
            let prev = *seen.get(&last).unwrap();
            seen.insert(last, i as u32 - 1);
            last = i as u32 - 1 - prev;
        } else {
            seen.insert(last, i as u32 - 1);
            last = 0;
        }
    }
    last
}

pub fn solve_p1(input: &mut Vec<u32>) -> u32 {
    solve(input, 2020)
}

pub fn solve_p2(input: &mut Vec<u32>) -> u32 {
    solve(input, 30_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_p1() {
        assert_eq!(436, solve_p1(&mut vec![0, 3, 6]));
    }
    #[test]
    fn check_p2() {
        assert_eq!(175594, solve_p2(&mut vec![0, 3, 6]));
    }
}
