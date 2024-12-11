use std::{collections::HashMap, time::Instant};

fn count_digits(number: usize) -> u32 {
    let mut digits = 0;

    while number / 10_usize.pow(digits) > 0 {
        digits += 1;
    }

    digits
}

fn split_digits(number: usize, digits: u32) -> (usize, usize) {
    let split = 10_usize.pow(digits);

    (number / split, number % split)
}

// naive approach
fn q1(input: &str) -> usize {
    let mut stones = input
        .split_whitespace()
        .map(|number| number.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    for _ in 0..25 {
        for i in 0..stones.len() {
            let current_number = stones[i];

            if current_number == 0 {
                stones[i] = 1;
            } else {
                let digits = count_digits(current_number);

                if digits % 2 == 0 {
                    let (left, right) = split_digits(current_number, digits / 2);
                    //order is useless
                    stones[i] = left;
                    stones.push(right);
                } else {
                    stones[i] = current_number * 2024;
                }
            }
        }
    }

    stones.len()
}

fn count_splits(number: usize, mem: &mut HashMap<(usize, usize), usize>, times: usize) -> usize {
    if times == 0 {
        return 0;
    }

    match mem.get(&(number, times)) {
        Some(splits) => return *splits,
        None => {
            let splits = if number == 0 {
                count_splits(1, mem, times - 1)
            } else {
                let digits = count_digits(number);

                if digits % 2 == 0 {
                    let (left, right) = split_digits(number, digits / 2);
                    let left_splits = count_splits(left, mem, times - 1);
                    let right_splits = count_splits(right, mem, times - 1);

                    left_splits + right_splits + 1
                } else {
                    count_splits(number * 2024, mem, times - 1)
                }
            };
            mem.insert((number, times), splits);
            return splits;
        }
    };
}

fn q2(input: &str) -> usize {
    let mut mem = HashMap::new();

    input
        .split_whitespace()
        .map(|number| number.parse::<usize>().unwrap())
        .map(|number| count_splits(number, &mut mem, 75) + 1)
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

    assert_eq!(q1(test), 55312);
    assert_eq!(q2(test), 65601038650482);
}
