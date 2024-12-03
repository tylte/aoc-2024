// Gange but is ok ig :)

fn q1(input: &str) -> usize {
    let ix = vec!['m', 'u', 'l', '(', ',', ')'];

    input
        .lines()
        .map(|line| {
            let mut res = 0;

            let mut index = 0;
            let mut left = 0;
            let mut right = 0;

            let mut number_chars = "".to_string();

            let mut chars = line.chars();

            while let Some(c) = chars.next() {
                if ix[index] == c {
                    if !number_chars.is_empty() {
                        if left == 0 {
                            left = number_chars.parse().unwrap();
                        } else {
                            right = number_chars.parse().unwrap();
                        }
                    }
                    index += 1;
                    number_chars = "".to_string();
                } else if c.is_digit(10) {
                    number_chars.push(c);
                } else {
                    index = 0;
                    number_chars = "".to_string();
                    left = 0;
                    right = 0;
                }

                if index == ix.len() {
                    res += left * right;
                    index = 0;
                    left = 0;
                    right = 0;
                }
            }

            res
        })
        .sum::<usize>()
}

fn q2(input: &str) -> usize {
    let ix = vec!['m', 'u', 'l', '(', ',', ')'];
    let do_ix = vec!['d', 'o', '(', ')'];
    let dont_ix = vec!['d', 'o', 'n', '\'', 't', '(', ')'];

    let mut is_enabled = true;

    input
        .lines()
        .map(|line| {
            let mut res = 0;

            let mut index = 0;
            let mut index_do = 0;
            let mut index_dont = 0;
            let mut left = 0;
            let mut right = 0;

            let mut number_chars = "".to_string();

            let mut chars = line.chars();

            while let Some(c) = chars.next() {
                if ix[index] == c {
                    if !number_chars.is_empty() {
                        if left == 0 {
                            left = number_chars.parse().unwrap();
                        } else {
                            right = number_chars.parse().unwrap();
                        }
                    }
                    index += 1;
                    number_chars = "".to_string();
                    index_dont = 0;
                    index_do = 0;
                } else if c.is_digit(10) {
                    number_chars.push(c);
                } else if do_ix[index_do] == c {
                    if c == 'd' || c == 'o' {
                        index_dont += 1;
                    }
                    index = 0;
                    index_do += 1;
                    left = 0;
                    number_chars = "".to_string();
                    right = 0;
                } else if dont_ix[index_dont] == c {
                    index_dont += 1;
                    index_do = 0;
                    index = 0;
                    left = 0;
                    right = 0;
                    number_chars = "".to_string();
                } else {
                    index = 0;
                    number_chars = "".to_string();
                    left = 0;
                    right = 0;
                    index_dont = 0;
                    index_do = 0;
                }

                if index_dont == dont_ix.len() {
                    is_enabled = false;
                    number_chars = "".to_string();
                    index = 0;
                    left = 0;
                    right = 0;
                    index_dont = 0;
                    index_do = 0;
                }

                if index_do == do_ix.len() {
                    is_enabled = true;
                    number_chars = "".to_string();
                    index = 0;
                    left = 0;
                    right = 0;
                    index_dont = 0;
                    index_do = 0;
                }

                if index == ix.len() {
                    if is_enabled {
                        res += left * right;
                    }
                    index = 0;
                    left = 0;
                    right = 0;
                    index_dont = 0;
                    index_do = 0;
                }
            }

            res
        })
        .sum::<usize>()
}

fn main() {
    let real = include_str!("../input.real.txt");

    dbg!(q1(real));
    dbg!(q2(real));
}

#[test]
fn test() {
    let test = include_str!("../input.test.txt");

    assert_eq!(q1(test), 161);
    assert_eq!(q2(test), 48);
}
