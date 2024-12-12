use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

const OFFSETS: [[isize; 2]; 4] = [[1, 0], [0, 1], [-1, 0], [0, -1]];

fn count_price(
    x: usize,
    y: usize,
    map: &[Vec<char>],
    visited: &mut HashSet<(usize, usize)>,
) -> usize {
    if !visited.insert((x, y)) {
        return 0;
    }
    let label = map[x][y];

    let mut q = VecDeque::new();
    q.push_back((x, y));

    let mut area = 0;
    let mut perimeter = 0;

    while !q.is_empty() {
        for _ in 0..q.len() {
            let (x, y) = q.pop_front().unwrap();

            area += 1;

            for [offset_x, offset_y] in OFFSETS {
                let Some(next_x) = x.checked_add_signed(offset_x) else {
                    perimeter += 1;
                    continue;
                };
                let Some(next_y) = y.checked_add_signed(offset_y) else {
                    perimeter += 1;
                    continue;
                };
                let Some(&next_label) = map
                    .get(next_x)
                    .map(|label_line| label_line.get(next_y))
                    .flatten()
                else {
                    perimeter += 1;
                    continue;
                };

                if next_label == label {
                    if !visited.contains(&(next_x, next_y)) {
                        visited.insert((next_x, next_y));
                        q.push_back((next_x, next_y));
                    }
                } else {
                    perimeter += 1;
                }
            }
        }
    }

    area * perimeter
}

fn q1(input: &str) -> usize {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut res = 0;
    let mut visited = HashSet::new();

    for x in 0..map.len() {
        for y in 0..map[x].len() {
            res += count_price(x, y, &map, &mut visited);
        }
    }

    res
}

fn count_bulk_price(
    x: usize,
    y: usize,
    map: &[Vec<char>],
    visited: &mut HashSet<(usize, usize)>,
) -> usize {
    if !visited.insert((x, y)) {
        return 0;
    }
    let label = map[x][y];

    let mut q = VecDeque::new();
    q.push_back((x, y));

    let mut area = 0;
    let mut dir_to_outside_coord = HashMap::new();

    while !q.is_empty() {
        for _ in 0..q.len() {
            let (x, y) = q.pop_front().unwrap();

            area += 1;

            for (dir, [offset_x, offset_y]) in OFFSETS.into_iter().enumerate() {
                let (Some(next_x), Some(next_y)) = (
                    x.checked_add_signed(offset_x),
                    y.checked_add_signed(offset_y),
                ) else {
                    dir_to_outside_coord
                        .entry(dir)
                        .or_insert_with(Vec::new)
                        .push((x, y));
                    continue;
                };

                let Some(&next_label) = map
                    .get(next_x)
                    .map(|label_line| label_line.get(next_y))
                    .flatten()
                else {
                    dir_to_outside_coord
                        .entry(dir)
                        .or_insert_with(Vec::new)
                        .push((x, y));
                    continue;
                };

                if next_label == label {
                    if !visited.contains(&(next_x, next_y)) {
                        visited.insert((next_x, next_y));
                        q.push_back((next_x, next_y));
                    }
                } else {
                    dir_to_outside_coord
                        .entry(dir)
                        .or_insert_with(Vec::new)
                        .push((x, y));
                }
            }
        }
    }

    let mut sides = 0;

    for (dir, mut coords) in dir_to_outside_coord {
        sides += 1;
        // dir = 0 : down | dir = 1 : right | dir = 2 : up | dir = 3 : left
        match dir {
            0 | 2 => {
                coords.sort_by(|(x_a, y_a), (x_b, y_b)| match x_a.cmp(x_b) {
                    Ordering::Less => Ordering::Less,
                    Ordering::Equal => y_a.cmp(y_b),
                    Ordering::Greater => Ordering::Greater,
                });

                let mut iter = coords.into_iter();

                // always atleast one
                let (mut last_x, mut last_y) = iter.next().unwrap();

                for (x, y) in iter {
                    if x != last_x || last_y != y - 1 {
                        // other side
                        sides += 1;
                    }

                    last_x = x;
                    last_y = y;
                }
            }
            1 | 3 => {
                coords.sort_by(|(x_a, y_a), (x_b, y_b)| match y_a.cmp(y_b) {
                    Ordering::Less => Ordering::Less,
                    Ordering::Equal => x_a.cmp(x_b),
                    Ordering::Greater => Ordering::Greater,
                });
                let mut iter = coords.into_iter();

                // always atleast one
                let (mut last_x, mut last_y) = iter.next().unwrap();

                for (x, y) in iter {
                    if y != last_y || last_x != x - 1 {
                        // other side
                        sides += 1;
                    }

                    last_x = x;
                    last_y = y;
                }
            }
            _ => unreachable!("no dir {dir}"),
        }
    }

    area * sides
}

fn q2(input: &str) -> usize {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut res = 0;
    let mut visited = HashSet::new();

    for x in 0..map.len() {
        for y in 0..map[x].len() {
            res += count_bulk_price(x, y, &map, &mut visited);
        }
    }

    res
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

    assert_eq!(q1(test), 1930);
    assert_eq!(q2(test), 1206);
}
