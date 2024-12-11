use std::{
    collections::{HashSet, VecDeque},
    time::Instant,
};

const OFFSETS: [[isize; 2]; 4] = [[1, 0], [0, 1], [-1, 0], [0, -1]];

fn count_trailhead_score(x: usize, y: usize, map: &[Vec<u32>]) -> usize {
    let mut current = map[x][y];

    assert!(current == 0, "position given is not a trailhead");

    let mut q = VecDeque::new();
    q.push_back((x, y));
    while !q.is_empty() {
        current += 1;
        for _ in 0..q.len() {
            let (current_x, current_y) = q.pop_front().unwrap();

            for [x_offset, y_offset] in OFFSETS {
                let Some(next_x) = current_x.checked_add_signed(x_offset) else {
                    continue;
                };

                let Some(next_y) = current_y.checked_add_signed(y_offset) else {
                    continue;
                };

                if let Some(&new_height) = map.get(next_x).map(|line| line.get(next_y)).flatten() {
                    if new_height == current {
                        q.push_back((next_x, next_y));
                    }
                }
            }
        }

        if current == 9 {
            return q.into_iter().collect::<HashSet<_>>().len();
        }
    }
    return 0;
}

fn count_trailhead_rating(x: usize, y: usize, map: &[Vec<u32>]) -> usize {
    let mut current = map[x][y];

    assert!(current == 0, "position given is not a trailhead");

    let mut q = VecDeque::new();
    q.push_back((x, y));
    while !q.is_empty() {
        current += 1;
        for _ in 0..q.len() {
            let (current_x, current_y) = q.pop_front().unwrap();

            for [x_offset, y_offset] in OFFSETS {
                let Some(next_x) = current_x.checked_add_signed(x_offset) else {
                    continue;
                };

                let Some(next_y) = current_y.checked_add_signed(y_offset) else {
                    continue;
                };

                if let Some(&new_height) = map.get(next_x).map(|line| line.get(next_y)).flatten() {
                    if new_height == current {
                        q.push_back((next_x, next_y));
                    }
                }
            }
        }

        if current == 9 {
            return q.len();
        }
    }
    return 0;
}

fn q1(input: &str) -> usize {
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    map.iter()
        .enumerate()
        .map(|(x, line)| {
            line.iter()
                .enumerate()
                .map(|(y, &height)| {
                    if height == 0 {
                        count_trailhead_score(x, y, &map)
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

fn q2(input: &str) -> usize {
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    map.iter()
        .enumerate()
        .map(|(x, line)| {
            line.iter()
                .enumerate()
                .map(|(y, &height)| {
                    if height == 0 {
                        count_trailhead_rating(x, y, &map)
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
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

    assert_eq!(q1(test), 36);
    assert_eq!(q2(test), 81);
}
