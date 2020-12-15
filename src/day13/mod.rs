fn parse(input: &str) -> (i32, Vec<(usize, i32)>) {
    if let [time, buses_text] = input.lines().collect::<Vec<_>>()[0..2] {
        (
            time.parse().unwrap(),
            buses_text
                .split(',')
                .enumerate()
                .filter_map(|(idx, s)| s.parse().ok().map(|num| (idx, num)))
                .collect::<Vec<_>>(),
        )
    } else {
        panic!("bruh")
    }
}

fn get_earliest_after(bus: i32, time: i32) -> i32 {
    let latest_before = time / bus * bus;
    if latest_before < time {
        latest_before + bus
    } else {
        latest_before
    }
}

pub fn solve_p1(input: &str) -> i32 {
    let (time, buses) = parse(input);
    let mut best = get_earliest_after(buses[0].1, time);
    let mut best_bus = buses[0].1;
    for bus in buses {
        let now = get_earliest_after(bus.1, time);
        if now < best {
            best = now;
            best_bus = bus.1;
        }
    }
    (best - time) * best_bus
}

pub fn solve_p2(input: &str) -> i64 {
    let (_, buses) = parse(input);
    let congruences = buses.iter().map(|(idx, v)| {
        let mut rem = v - *idx as i32;
        while rem < 0 {
            rem += v;
        }
        (rem % v, v)
    });
    let mut start: i64 = 0;
    let mut acc: i64 = 1;
    for (rem, v) in congruences {
        while start % *v as i64 != rem as i64 {
            start += acc
        }
        acc *= *v as i64
    }
    start
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_INPUT: &str = "939
7,13,x,x,59,x,31,19";

    static SAMPLE_INPUT2: &str = "939
67,x,7,59,61";

    static SAMPLE_INPUT3: &str = "939
67,7,x,59,61";

    static SAMPLE_INPUT4: &str = "939
1789,37,47,1889";

    #[test]
    fn check_parser() {
        assert_eq!(
            (939, vec![(0, 7), (1, 13), (4, 59), (6, 31), (7, 19)]),
            parse(SAMPLE_INPUT)
        );
    }

    #[test]
    fn check_p1() {
        assert_eq!(295, solve_p1(SAMPLE_INPUT));
    }

    #[test]
    fn check_p2() {
        assert_eq!(1068781, solve_p2(SAMPLE_INPUT));
        assert_eq!(1261476, solve_p2(SAMPLE_INPUT3));
        assert_eq!(779210, solve_p2(SAMPLE_INPUT2));
        assert_eq!(1202161486, solve_p2(SAMPLE_INPUT4));
    }
}
