use std::{collections::HashMap, time::Instant};

fn q1(input: &str) -> usize {
    let (ordering, produce) = input.split_once("\n\n").unwrap();

    let predecessor_to_numbers: HashMap<usize, Vec<usize>> =
        ordering
            .lines()
            .fold(HashMap::new(), |mut predecessor_to_numbers, line| {
                let (predecessor, number) = line.split_once('|').unwrap();

                let predecessor: usize = predecessor.parse().unwrap();

                let number: usize = number.parse().unwrap();

                predecessor_to_numbers
                    .entry(predecessor)
                    .or_insert_with(Vec::new)
                    .push(number);

                predecessor_to_numbers
            });

    let mut solution = 0;

    let mut buffer = vec![];

    'outer: for line in produce.lines() {
        buffer.clear();
        for number in line
            .split(",")
            .map(|number| number.parse::<usize>().unwrap())
        {
            if let Some(forbidden_numbers) = predecessor_to_numbers.get(&number) {
                if forbidden_numbers
                    .iter()
                    .any(|forbidden_number| buffer.contains(forbidden_number))
                {
                    continue 'outer;
                }
            }
            buffer.push(number);
        }
        solution += buffer[buffer.len() / 2];
    }

    solution
}

fn q2(input: &str) -> usize {
    let (ordering, produce) = input.split_once("\n\n").unwrap();

    let predecessor_to_numbers: HashMap<usize, Vec<usize>> =
        ordering
            .lines()
            .fold(HashMap::new(), |mut predecessor_to_numbers, line| {
                let (predecessor, number) = line.split_once('|').unwrap();

                let predecessor: usize = predecessor.parse().unwrap();

                let number: usize = number.parse().unwrap();

                predecessor_to_numbers
                    .entry(predecessor)
                    .or_insert_with(Vec::new)
                    .push(number);

                predecessor_to_numbers
            });

    let mut solution = 0;

    let mut buffer = vec![];

    for line in produce.lines() {
        buffer.clear();
        let mut is_unordered = false;
        for number in line
            .split(",")
            .map(|number| number.parse::<usize>().unwrap())
        {
            let mut new_pos = None;
            if let Some(forbidden_numbers) = predecessor_to_numbers.get(&number) {
                new_pos = forbidden_numbers
                    .iter()
                    .filter_map(|forbidden_number| {
                        buffer.iter().position(|number| number == forbidden_number)
                    })
                    .min();
            }
            if let Some(new_pos) = new_pos {
                is_unordered = true;
                buffer.insert(new_pos, number);
            } else {
                buffer.push(number);
            }
        }
        if is_unordered {
            solution += buffer[buffer.len() / 2];
        }
    }

    solution
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

    assert_eq!(q1(test), 143);
    assert_eq!(q2(test), 123);
}
