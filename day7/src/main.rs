use std::time::Instant;

fn find_target(target: usize, current: usize, idx: usize, elements: &[usize]) -> bool {
    if let Some(number) = elements.get(idx) {
        let add_result = current + number;
        let mul_result = current * number;

        return find_target(target, add_result, idx + 1, elements)
            || find_target(target, mul_result, idx + 1, elements);
    }

    current == target
}

fn find_target_q2(target: usize, current: usize, idx: usize, elements: &[usize]) -> bool {
    if let Some(&number) = elements.get(idx) {
        let add_result = number + current;
        let mul_result = number * current;
        let concat_result = concatenate_numbers(current, number);

        return find_target_q2(target, add_result, idx + 1, elements)
            || find_target_q2(target, mul_result, idx + 1, elements)
            || find_target_q2(target, concat_result, idx + 1, elements);
    }

    current == target
}

fn concatenate_numbers(a: usize, b: usize) -> usize {
    let mut b_len = 0;

    while (b / (10_usize.pow(b_len))) > 0 {
        b_len += 1;
    }
    return (a * 10_usize.pow(b_len)) + b;
}

fn q1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (target, elements) = line
                .split_once(":")
                .map(|(target, elems)| {
                    (
                        target.parse::<usize>().unwrap(),
                        elems
                            .split_whitespace()
                            .map(|number| number.parse::<usize>().unwrap())
                            .collect::<Vec<_>>(),
                    )
                })
                .unwrap();

            if !elements.is_empty() && find_target(target, elements[0], 1, &elements) {
                target
            } else {
                0
            }
        })
        .sum()
}

fn q2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (target, elements) = line
                .split_once(":")
                .map(|(target, elems)| {
                    (
                        target.parse::<usize>().unwrap(),
                        elems
                            .split_whitespace()
                            .map(|number| number.parse::<usize>().unwrap())
                            .collect::<Vec<_>>(),
                    )
                })
                .unwrap();

            if !elements.is_empty() && find_target_q2(target, elements[0], 1, &elements) {
                target
            } else {
                0
            }
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

    assert_eq!(q1(test), 3749);
    assert_eq!(q2(test), 11387);
}
