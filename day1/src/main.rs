use std::collections::{BinaryHeap, HashMap};

fn q1(input: &str) -> usize {
    let (mut left_heap, mut right_heap) = input.lines().filter(|line| !line.is_empty()).fold(
        (BinaryHeap::new(), BinaryHeap::new()),
        |(mut left_heap, mut right_heap), line| {
            let mut iter = line.split_whitespace();
            let left: usize = iter.next().unwrap().parse().unwrap();
            let right: usize = iter.next().unwrap().parse().unwrap();

            left_heap.push(left);
            right_heap.push(right);

            (left_heap, right_heap)
        },
    );

    let mut sum = 0;

    while let Some(left) = left_heap.pop() {
        let right = right_heap.pop().unwrap();
        sum += left.abs_diff(right);
    }

    sum
}

fn q2(input: &str) -> usize {
    let (numbers, number_to_occurence) = input.lines().filter(|line| !line.is_empty()).fold(
        (Vec::new(), HashMap::new()),
        |(mut numbers, mut number_to_occurence), line| {
            let mut iter = line.split_whitespace();
            let left: usize = iter.next().unwrap().parse().unwrap();
            let right: usize = iter.next().unwrap().parse().unwrap();

            numbers.push(left);
            *number_to_occurence.entry(right).or_insert(0 as usize) += 1;

            (numbers, number_to_occurence)
        },
    );

    let mut sum = 0;

    for number in numbers {
        sum += number_to_occurence
            .get(&number)
            .copied()
            .unwrap_or_default()
            * number;
    }

    sum
}

fn main() {
    let test_input = include_str!("../input.test.txt");
    let real_input = include_str!("../input.real.txt");

    dbg!(q1(test_input));
    dbg!(q1(real_input));

    dbg!(q2(test_input));
    dbg!(q2(real_input));
}
