#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum GridStatus {
    Floor,
    Empty,
    Taken,
}

fn to_grid_status(input: &str) -> Vec<Vec<GridStatus>> {
    input
        .split_whitespace()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '.' => GridStatus::Floor,
                    'L' => GridStatus::Empty,
                    '#' => GridStatus::Taken,
                    _ => panic!("bruh"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn neighbour_coords_for(grid: &[Vec<GridStatus>]) -> Vec<Vec<Vec<(usize, usize)>>> {
    (0..grid.len())
        .map(|y| {
            (0..grid[0].len())
                .map(|x| {
                    (((y as i32 - 1).max(0) as usize)..(y + 2).min(grid.len()))
                        .flat_map(|row| {
                            (((x as i32 - 1).max(0) as usize)..(x + 2).min(grid[0].len()))
                                .flat_map(|col| {
                                    if row == y && col == x {
                                        vec![]
                                    } else {
                                        vec![(row, col)]
                                    }
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn visible_neighbour_coords_for(grid: &[Vec<GridStatus>]) -> Vec<Vec<Vec<(usize, usize)>>> {
    (0..grid.len())
        .map(|y| {
            (0..grid[0].len())
                .map(|x| {
                    let mut coords = vec![];
                    for (vert, hori) in &[
                        (-1_i32, -1_i32),
                        (-1, 0),
                        (-1, 1),
                        (0, -1),
                        (0, 1),
                        (1, -1),
                        (1, 0),
                        (1, 1),
                    ] {
                        let (mut cur_y, mut cur_x) = (y as i32 + vert, x as i32 + hori);
                        while cur_y >= 0
                            && cur_y < (grid.len() as i32)
                            && cur_x >= 0
                            && cur_x < (grid[0].len() as i32)
                        {
                            if grid[cur_y as usize][cur_x as usize] != GridStatus::Floor {
                                coords.push((cur_y as usize, cur_x as usize));
                                break;
                            }
                            cur_y += vert;
                            cur_x += hori;
                        }
                    }
                    coords
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn count_neighbours(
    grid: &[Vec<GridStatus>],
    neighbour_coords: &[Vec<Vec<(usize, usize)>>],
    x: usize,
    y: usize,
) -> i32 {
    neighbour_coords[y][x]
        .iter()
        .map(|(row, col)| {
            match grid
                .get(*row as usize)
                .and_then(|row| row.get(*col as usize))
            {
                Some(GridStatus::Taken) => 1,
                Some(GridStatus::Empty) | Some(GridStatus::Floor) | None => 0,
            }
        })
        .sum()
}

fn pretty_print(grid: &[Vec<GridStatus>]) {
    for line in grid.iter() {
        println!(
            "{}",
            line.iter()
                .map(|st| match st {
                    GridStatus::Empty => 'L',
                    GridStatus::Taken => '#',
                    GridStatus::Floor => '.',
                })
                .collect::<String>()
        );
    }
}

fn sim_one_round(
    front: &[Vec<GridStatus>],
    back: &mut [Vec<GridStatus>],
    neighbour_coords: &[Vec<Vec<(usize, usize)>>],
    threshold: i32,
) {
    for row in 0..front.len() {
        for col in 0..front[0].len() {
            let adjacent = count_neighbours(front, neighbour_coords, col, row);
            if front[row][col] == GridStatus::Empty && adjacent == 0 {
                back[row][col] = GridStatus::Taken;
            } else if front[row][col] == GridStatus::Taken && adjacent >= threshold {
                back[row][col] = GridStatus::Empty;
            } else {
                back[row][col] = front[row][col];
            }
        }
    }
}

fn same_state(left: &[Vec<GridStatus>], right: &[Vec<GridStatus>]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .all(|(l, r)| l.len() == r.len() && l.iter().zip(r).all(|(a, b)| a == b))
}

fn count_taken(grid: &[Vec<GridStatus>]) -> i32 {
    grid.iter()
        .map(|row| {
            row.iter()
                .map(|cell| match cell {
                    GridStatus::Taken => 1,
                    GridStatus::Empty | GridStatus::Floor => 0,
                })
                .sum::<i32>()
        })
        .sum()
}

pub fn solve_p1(input: &str) -> i32 {
    let mut front = &mut to_grid_status(input);
    let mut back = &mut vec![vec![GridStatus::Floor; front[0].len()]; front.len()];
    let neighbour_coords = neighbour_coords_for(&front);
    // let mut round = 1;
    while !same_state(front, back) {
        sim_one_round(&front, back, &neighbour_coords, 4);
        std::mem::swap(&mut front, &mut back);
        // println!("\nresult of round {}: ---", round);
        // round += 1;
        // pretty_print(&front);
    }
    count_taken(front)
}

pub fn solve_p2(input: &str) -> i32 {
    let mut front = &mut to_grid_status(input);
    let mut back = &mut vec![vec![GridStatus::Floor; front[0].len()]; front.len()];
    let neighbour_coords = visible_neighbour_coords_for(&front);
    // let mut round = 1;
    while !same_state(front, back) {
        sim_one_round(&front, back, &neighbour_coords, 5);
        std::mem::swap(&mut front, &mut back);
        // println!("\nresult of round {}: ---", round);
        // round += 1;
        // pretty_print(&front);
    }
    count_taken(front)
}

#[cfg(test)]
mod tests {
    use super::*;

    static SMALL_STATE: &str = "L.L
L##
.L#";

    static STARTING_STATE: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn convert_to_enums() {
        use GridStatus::*;
        assert_eq!(
            vec![
                vec![Empty, Floor, Empty],
                vec![Empty, Taken, Taken],
                vec![Floor, Empty, Taken]
            ],
            to_grid_status(SMALL_STATE)
        );
    }

    #[test]
    fn neighbour_coords() {
        assert_eq!(
            vec![(0, 1), (1, 0), (1, 1)],
            neighbour_coords_for(&to_grid_status(SMALL_STATE))[0][0]
        );
        assert_eq!(
            vec![
                (0, 0),
                (0, 1),
                (0, 2),
                (1, 0),
                (1, 2),
                (2, 0),
                (2, 1),
                (2, 2)
            ],
            neighbour_coords_for(&to_grid_status(SMALL_STATE))[1][1]
        );
    }

    #[test]
    fn neighbour_counts() {
        let grid_status = to_grid_status(SMALL_STATE);
        let neighbour_coords = neighbour_coords_for(&grid_status);
        assert_eq!(3, count_neighbours(&grid_status, &neighbour_coords, 1, 2));
        assert_eq!(2, count_neighbours(&grid_status, &neighbour_coords, 1, 0));
    }

    #[test]
    fn solve_first_part() {
        assert_eq!(37, solve_p1(STARTING_STATE));
    }

    #[test]
    fn find_nearest_seat() {
        let sample: &str = "
.......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....";
        let grid = to_grid_status(sample);
        println!("{:?}", grid);
        let visible_neighbours = visible_neighbour_coords_for(&grid);
        assert_eq!(
            vec![
                (2, 1),
                (1, 3),
                (0, 7),
                (4, 2),
                (4, 8),
                (7, 0),
                (8, 3),
                (5, 4)
            ],
            visible_neighbours[4][3]
        );
    }

    #[test]
    fn solve_second_part() {
        assert_eq!(26, solve_p2(STARTING_STATE));
    }
}
