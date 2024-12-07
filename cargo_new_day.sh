#!/bin/sh

# Check if the first parameter exists
if [ -z "$1" ]; then
  echo "No parameter provided. Please provide the day !"
  exit 1
fi

# Check if the parameter is a number
if ! [[ "$1" =~ ^[0-9]+$ ]]; then
  echo "The parameter is not a number. Exiting."
  exit 1
fi


cargo new day$1;
cd day$1/
touch input.test.txt;
touch input.real.txt;

cat <<EOF > "src/main.rs"
use std::time::Instant;

fn q1(input: &str) -> usize {
    todo!()
}

fn q2(input: &str) -> usize {
    todo!()
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

    assert_eq!(q1(test), 0);
    // assert_eq!(q2(test), 0);
}
EOF
