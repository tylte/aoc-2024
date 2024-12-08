use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

fn q1(input: &str) -> usize {
    let (freq_to_coords, max_x, max_y) = input.lines().enumerate().fold(
        (HashMap::new(), 0, 0),
        |(mut freq_to_coords, mut max_x, mut max_y), (x, line)| {
            max_x = max_x.max(x as isize);

            for (y, c) in line.chars().enumerate() {
                if c != '.' {
                    freq_to_coords
                        .entry(c)
                        .or_insert_with(Vec::new)
                        .push((x as isize, y as isize));
                }
                max_y = max_y.max(y as isize);
            }

            (freq_to_coords, max_x, max_y)
        },
    );

    let mut solution = HashSet::new();

    for (_, coords) in &freq_to_coords {
        for (current_x, current_y) in coords {
            for (other_x, other_y) in coords
                .iter()
                .filter(|(x, y)| x != current_x && y != current_y)
            {
                let anti_x = other_x + (other_x - current_x);
                let anti_y = other_y + (other_y - current_y);

                if anti_x >= 0 && anti_x <= max_x && anti_y >= 0 && anti_y <= max_y {
                    solution.insert((anti_x, anti_y));
                }
            }
        }
    }

    solution.len()
}

fn q2(input: &str) -> usize {
    let (freq_to_coords, max_x, max_y) = input.lines().enumerate().fold(
        (HashMap::new(), 0, 0),
        |(mut freq_to_coords, mut max_x, mut max_y), (x, line)| {
            max_x = max_x.max(x as isize);

            for (y, c) in line.chars().enumerate() {
                if c != '.' {
                    freq_to_coords
                        .entry(c)
                        .or_insert_with(Vec::new)
                        .push((x as isize, y as isize));
                }
                max_y = max_y.max(y as isize);
            }

            (freq_to_coords, max_x, max_y)
        },
    );

    let mut solution = HashSet::new();

    for (_, coords) in &freq_to_coords {
        if coords.len() > 1 {
            solution.extend(coords.iter())
        };
        for (current_x, current_y) in coords {
            for (other_x, other_y) in coords
                .iter()
                .filter(|(x, y)| x != current_x && y != current_y)
            {
                let diff_x = other_x - current_x;
                let diff_y = other_y - current_y;

                let mut anti_x = other_x + diff_x;
                let mut anti_y = other_y + diff_y;

                while anti_x >= 0 && anti_x <= max_x && anti_y >= 0 && anti_y <= max_y {
                    solution.insert((anti_x, anti_y));
                    anti_x += diff_x;
                    anti_y += diff_y;
                }
            }
        }
    }

    solution.len()
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

    assert_eq!(q1(test), 14);
    assert_eq!(q2(test), 34);
}
