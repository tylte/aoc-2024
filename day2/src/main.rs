fn q1(input: &str) -> usize {
    input
        .lines()
        .filter(|&line| !line.is_empty() && is_safe(line))
        .count()
}

fn is_safe(line: &str) -> bool {
    let mut iter = line
        .split_whitespace()
        .map(|nb| nb.parse::<usize>().unwrap());

    let mut is_increasing: Option<bool> = None;

    let mut last_number = iter.next().unwrap();

    while let Some(current_number) = iter.next() {
        let diff = current_number.abs_diff(last_number);

        if diff == 0 || diff > 3 {
            return false;
        }

        let is_current_greater = current_number > last_number;
        match is_increasing {
            Some(is_increasing)
                if (is_increasing && !is_current_greater)
                    || (!is_increasing && is_current_greater) =>
            {
                return false;
            }
            None => is_increasing = Some(is_current_greater),
            _ => {}
        }
        last_number = current_number;
    }

    true
}

fn q2(input: &str) -> usize {
    input
        .lines()
        .filter(|&line| {
            !line.is_empty() && (0..line.len()).any(|idx_to_skip| is_safe_q2(line, idx_to_skip))
        })
        .count()
}

fn is_safe_q2(line: &str, idx_to_skip: usize) -> bool {
    let mut iter = line
        .split_whitespace()
        .enumerate()
        .filter(|&(idx, _)| idx != idx_to_skip)
        .map(|(_, nb)| nb.parse::<usize>().unwrap());

    let mut is_increasing: Option<bool> = None;

    let mut last_number = iter.next().unwrap();

    while let Some(current_number) = iter.next() {
        let diff = current_number.abs_diff(last_number);

        if diff == 0 || diff > 3 {
            return false;
        }

        let is_current_greater = current_number > last_number;
        match is_increasing {
            Some(is_increasing)
                if (is_increasing && !is_current_greater)
                    || (!is_increasing && is_current_greater) =>
            {
                return false;
            }
            None => is_increasing = Some(is_current_greater),
            _ => {}
        }
        last_number = current_number;
    }

    true
}

fn main() {
    let input_real = include_str!("../input.real.txt");

    dbg!(q1(input_real));
    dbg!(q2(input_real));
}

#[cfg(test)]
mod test {
    use crate::{q1, q2};

    #[test]
    fn test() {
        let input_test = include_str!("../input.test.txt");

        assert_eq!(q1(input_test), 2);
        assert_eq!(q2(input_test), 4);
    }
}
