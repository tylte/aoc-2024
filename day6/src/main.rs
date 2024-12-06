use std::{collections::HashSet, time::Instant};

fn q1(input: &str) -> usize {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (mut x, mut y) = grid
        .iter()
        .enumerate()
        .filter_map(|(idx, line)| {
            line.iter()
                .position(|&c| c == '^')
                .map(|idx_y| (idx, idx_y))
        })
        .next()
        .unwrap();

    let mut next_orientation = ['>', 'v', '<', '^'].iter().cycle();

    loop {
        let guard = grid[x][y];
        grid[x][y] = 'x';
        let mut next_x = x;
        let mut next_y = y;
        match guard {
            '>' => {
                if next_y == grid[next_x].len() - 1 {
                    break;
                } else {
                    next_y += 1;
                }
            }
            'v' => {
                if next_x == grid.len() - 1 {
                    break;
                } else {
                    next_x += 1;
                }
            }
            '<' => {
                if next_y == 0 {
                    break;
                } else {
                    next_y -= 1;
                }
            }
            '^' => {
                if next_x == 0 {
                    break;
                } else {
                    next_x -= 1;
                }
            }
            _ => unreachable!(),
        }
        let next_char = grid[next_x][next_y];
        if next_char == '#' {
            grid[x][y] = *next_orientation.next().unwrap();
        } else {
            x = next_x;
            y = next_y;
            grid[x][y] = guard;
        }
    }

    grid.iter()
        .map(|line| line.iter().filter(|&&c| c == 'x').count())
        .sum()
}

fn q2(input: &str) -> usize {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (starting_x, starting_y) = grid
        .iter()
        .enumerate()
        .filter_map(|(idx, line)| {
            line.iter()
                .position(|&c| c == '^')
                .map(|idx_y| (idx, idx_y))
        })
        .next()
        .unwrap();
    let mut possible_new_obstacle = 0;

    // easily optimizable: don't use hashset and make a first run to get the path, then add the
    // obstable only on the path, other locations are useless.
    let mut visited = HashSet::new();

    for new_obstacle_x in 0..grid.len() {
        for new_obstacle_y in 0..grid[new_obstacle_x].len() {
            grid[starting_x][starting_y] = '^';
            if grid[new_obstacle_x][new_obstacle_y] != '.' {
                continue;
            }

            let mut next_orientation = ['>', 'v', '<', '^'].iter().cycle();
            let mut x = starting_x;
            let mut y = starting_y;
            grid[new_obstacle_x][new_obstacle_y] = '#';
            visited.clear();
            loop {
                let guard = grid[x][y];
                grid[x][y] = '.';
                let mut next_x = x;
                let mut next_y = y;
                if !visited.insert((guard, x, y)) {
                    possible_new_obstacle += 1;
                    break;
                }
                match guard {
                    '>' => {
                        if next_y == grid[next_x].len() - 1 {
                            break;
                        } else {
                            next_y += 1;
                        }
                    }
                    'v' => {
                        if next_x == grid.len() - 1 {
                            break;
                        } else {
                            next_x += 1;
                        }
                    }
                    '<' => {
                        if next_y == 0 {
                            break;
                        } else {
                            next_y -= 1;
                        }
                    }
                    '^' => {
                        if next_x == 0 {
                            break;
                        } else {
                            next_x -= 1;
                        }
                    }
                    _ => unreachable!(),
                }
                let next_char = grid[next_x][next_y];
                if next_char == '#' {
                    grid[x][y] = *next_orientation.next().unwrap();
                } else {
                    x = next_x;
                    y = next_y;
                    grid[x][y] = guard;
                }
            }
            grid[new_obstacle_x][new_obstacle_y] = '.';
        }
    }

    possible_new_obstacle
}

fn main() {
    let real = include_str!("../input.real.txt");
    let now = Instant::now();
    let sol_q1 = q1(real);
    println!("q1: {sol_q1} | {:?}", now.elapsed());
    let now = Instant::now();
    let sol_q2 = q2(real);
    println!("q2: {sol_q2} | {:?}", now.elapsed());
}
#[test]
fn test() {
    let test = include_str!("../input.test.txt");
    assert_eq!(q1(test), 41);
    assert_eq!(q2(test), 6);
}
