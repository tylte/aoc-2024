use std::{collections::HashSet, time::Instant};

fn q1(input: &str) -> usize {
    let (map, moves) = input.split_once("\n\n").unwrap();

    let mut map = map
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let moves = moves
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .flatten()
        .collect::<Vec<_>>();

    let (start_x, start_y) = map
        .iter()
        .enumerate()
        .find_map(|(x, line)| {
            line.iter()
                .enumerate()
                .find_map(|(y, &c)| if c == '@' { Some((x, y)) } else { None })
        })
        .unwrap();

    let (mut x, mut y) = (start_x, start_y);

    for movement in moves {
        let next_position = match movement {
            '^' => |x: usize, y: usize| Some((x.checked_sub(1)?, y)),
            '>' => |x: usize, y: usize| Some((x, y.checked_add(1)?)),
            '<' => |x: usize, y: usize| Some((x, y.checked_sub(1)?)),
            'v' => |x: usize, y: usize| Some((x.checked_add(1)?, y)),
            _ => unreachable!("illegal move {movement}"),
        };

        let (mut next_x, mut next_y) = (x, y);
        let mut swaps = vec![(next_x, next_y)];
        let is_legit_move = loop {
            let Some(next_pos) = next_position(next_x, next_y) else {
                break false;
            };
            (next_x, next_y) = next_pos;

            swaps.push((next_x, next_y));
            let Some(c) = map
                .get(next_x)
                .map(|line| line.get(next_y).copied())
                .flatten()
            else {
                break false;
            };

            if c == '#' {
                break false;
            }

            if c == '.' {
                break true;
            }
        };

        if is_legit_move {
            for ((x_a, y_a), (x_b, y_b)) in swaps
                .iter()
                .rev()
                .copied()
                .zip(swaps.iter().rev().copied().skip(1))
            {
                let c = map[x_a][y_a];
                map[x_a][y_a] = map[x_b][y_b];
                map[x_b][y_b] = c;
            }
            (x, y) = next_position(x, y).unwrap();
        }
    }

    map.into_iter()
        .enumerate()
        .map(|(x, line)| {
            line.into_iter()
                .enumerate()
                .map(|(y, c)| if c == 'O' { 100 * x + y } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

fn find_swaps<F>(
    map: &[Vec<char>],
    x: usize,
    y: usize,
    box_pushed: &mut HashSet<(usize, usize)>,
    next_position: F,
) -> Option<Vec<Vec<(usize, usize)>>>
where
    F: Fn(usize, usize) -> Option<(usize, usize)> + Clone,
{
    let (mut next_x, mut next_y) = (x, y);
    let mut all_swaps = vec![];

    let mut swaps = vec![(next_x, next_y)];
    loop {
        let (x, y) = next_position(next_x, next_y)?;

        swaps.push((x, y));
        let c = map.get(x).map(|line| line.get(y).copied()).flatten()?;

        if next_x != x {
            if c == '[' {
                if box_pushed.insert((x, y + 1)) {
                    let swaps = find_swaps(map, x, y + 1, box_pushed, next_position.clone())?;
                    all_swaps.extend(swaps);
                }
            } else if c == ']' {
                if box_pushed.insert((x, y - 1)) {
                    let swaps = find_swaps(map, x, y - 1, box_pushed, next_position.clone())?;
                    all_swaps.extend(swaps);
                }
            }
        }

        if c == '#' {
            return None;
        }

        if c == '.' {
            break;
        }
        (next_x, next_y) = (x, y);
    }

    all_swaps.push(swaps);

    let all_swaps = all_swaps
        .iter()
        .enumerate()
        .filter(|(idx, swaps)| {
            !all_swaps
                .iter()
                .enumerate()
                .filter(|(idx_b, _)| idx_b != idx)
                .any(|(_, swaps_b)| swaps.iter().all(|swap_a| swaps_b.contains(&swap_a)))
        })
        .map(|(_, swaps)| swaps.to_vec())
        .collect::<Vec<_>>();

    return Some(all_swaps);
}

fn q2(input: &str) -> usize {
    let (map, moves) = input.split_once("\n\n").unwrap();

    let mut map = map
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    if c == '#' {
                        ['#', '#']
                    } else if c == 'O' {
                        ['[', ']']
                    } else if c == '.' {
                        ['.', '.']
                    } else if c == '@' {
                        ['@', '.']
                    } else {
                        unreachable!()
                    }
                })
                .flatten()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let moves = moves
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .flatten()
        .collect::<Vec<_>>();

    let (start_x, start_y) = map
        .iter()
        .enumerate()
        .find_map(|(x, line)| {
            line.iter()
                .enumerate()
                .find_map(|(y, &c)| if c == '@' { Some((x, y)) } else { None })
        })
        .unwrap();

    let (mut x, mut y) = (start_x, start_y);

    for movement in moves {
        let next_position = match movement {
            '^' => |x: usize, y: usize| Some((x.checked_sub(1)?, y)),
            '>' => |x: usize, y: usize| Some((x, y.checked_add(1)?)),
            '<' => |x: usize, y: usize| Some((x, y.checked_sub(1)?)),
            'v' => |x: usize, y: usize| Some((x.checked_add(1)?, y)),
            _ => unreachable!("illegal move {movement}"),
        };

        let Some(multiple_swaps) = find_swaps(&map, x, y, &mut HashSet::new(), next_position)
        else {
            continue;
        };

        for swaps in multiple_swaps {
            for ((x_a, y_a), (x_b, y_b)) in swaps
                .iter()
                .rev()
                .copied()
                .zip(swaps.iter().rev().copied().skip(1))
            {
                let c = map[x_a][y_a];
                map[x_a][y_a] = map[x_b][y_b];
                map[x_b][y_b] = c;
            }
        }
        (x, y) = next_position(x, y).unwrap();
    }

    map.into_iter()
        .enumerate()
        .map(|(x, line)| {
            line.into_iter()
                .enumerate()
                .map(|(y, c)| if c == '[' { 100 * x + y } else { 0 })
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
    let small_test = include_str!("../input.smalltest.txt");
    let small_test_2 = include_str!("../input.smalltest2.txt");
    let test = include_str!("../input.test.txt");

    assert_eq!(q1(small_test), 2028);
    assert_eq!(q1(test), 10092);
    assert_eq!(q2(small_test_2), 618);
    assert_eq!(q2(test), 9021);
}
