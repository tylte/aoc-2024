use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Orientation {
    North,
    South,
    West,
    East,
}

impl Orientation {
    pub fn next_positions_with_cost(
        &self,
        map: &[Vec<char>],
        x: usize,
        y: usize,
    ) -> Vec<(usize, usize, usize, Orientation)> {
        let offsets_with_cost_and_orientation = match self {
            Orientation::North => [
                (-1, 0, 1, Self::North),
                (0, 1, 1001, Self::East),
                (0, -1, 1001, Self::West),
            ],
            Orientation::South => [
                (1, 0, 1, Self::South),
                (0, 1, 1001, Self::East),
                (0, -1, 1001, Self::West),
            ],
            Orientation::West => [
                (0, -1, 1, Self::West),
                (1, 0, 1001, Self::South),
                (-1, 0, 1001, Self::North),
            ],
            Orientation::East => [
                (0, 1, 1, Self::East),
                (1, 0, 1001, Self::South),
                (-1, 0, 1001, Self::North),
            ],
        };

        offsets_with_cost_and_orientation
            .into_iter()
            .filter_map(|(offset_x, offset_y, cost, orientation)| {
                let (next_x, next_y) = (
                    x.checked_add_signed(offset_x)?,
                    y.checked_add_signed(offset_y)?,
                );
                let next_char = map.get(next_x).map(|line| line.get(next_y)).flatten()?;
                if *next_char == '#' {
                    None
                } else {
                    Some((next_x, next_y, cost, orientation))
                }
            })
            .collect()
    }
}

fn q1(input: &str) -> usize {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (start_x, start_y) = map
        .iter()
        .enumerate()
        .find_map(|(x, line)| {
            line.iter()
                .enumerate()
                .find_map(|(y, c)| if *c == 'S' { Some(y) } else { None })
                .map(|y| (x, y))
        })
        .unwrap();

    let mut q = VecDeque::new();
    q.push_back((start_x, start_y, 0, Orientation::East));
    let mut visited: HashMap<(usize, usize, Orientation), usize> = HashMap::new();

    let mut sol = usize::MAX;

    while !q.is_empty() {
        for _ in 0..q.len() {
            let (x, y, current_cost, orientation) = q.pop_front().unwrap();

            for (next_x, next_y, cost, next_orientation) in
                orientation.next_positions_with_cost(&map, x, y)
            {
                let new_cost = cost + current_cost;
                if map[next_x][next_y] == 'E' {
                    if sol > new_cost {
                        sol = new_cost;
                    }
                } else {
                    if sol > new_cost {
                        let entry = visited
                            .entry((next_x, next_y, next_orientation))
                            .or_insert(usize::MAX);

                        if *entry > new_cost {
                            *entry = new_cost;
                            visited.insert((next_x, next_y, next_orientation), new_cost);
                            q.push_back((next_x, next_y, new_cost, next_orientation));
                        }
                    }
                }
            }
        }
    }

    sol
}

fn q2(input: &str) -> usize {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (start_x, start_y) = map
        .iter()
        .enumerate()
        .find_map(|(x, line)| {
            line.iter()
                .enumerate()
                .find_map(|(y, c)| if *c == 'S' { Some(y) } else { None })
                .map(|y| (x, y))
        })
        .unwrap();

    let mut q = VecDeque::new();
    q.push_back((start_x, start_y, 0, Orientation::East, Vec::new()));
    let mut visited: HashMap<(usize, usize, Orientation), usize> = HashMap::new();

    let mut cost_to_visited: HashMap<usize, HashSet<(usize, usize)>> = HashMap::new();

    let mut sol = usize::MAX;

    while !q.is_empty() {
        for _ in 0..q.len() {
            let (x, y, current_cost, orientation, mut path) = q.pop_front().unwrap();
            path.push((x, y));

            for (next_x, next_y, cost, next_orientation) in
                orientation.next_positions_with_cost(&map, x, y)
            {
                let new_cost = cost + current_cost;
                if map[next_x][next_y] == 'E' {
                    if sol >= new_cost {
                        let visited = cost_to_visited.entry(new_cost).or_insert_with(HashSet::new);

                        visited.extend(&path);
                        visited.insert((next_x, next_y));

                        sol = new_cost;
                    }
                } else {
                    if sol >= new_cost {
                        let entry = visited
                            .entry((next_x, next_y, next_orientation))
                            .or_insert(usize::MAX);

                        if *entry >= new_cost {
                            *entry = new_cost;
                            visited.insert((next_x, next_y, next_orientation), new_cost);
                            q.push_back((
                                next_x,
                                next_y,
                                new_cost,
                                next_orientation,
                                path.to_vec(),
                            ));
                        }
                    }
                }
            }
        }
    }

    cost_to_visited.get(&sol).unwrap().len()
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
    let test2 = include_str!("../input.test2.txt");

    assert_eq!(q1(test), 11048);
    assert_eq!(q1(test2), 7036);
    assert_eq!(q2(test), 64);
    assert_eq!(q2(test2), 45);
}
