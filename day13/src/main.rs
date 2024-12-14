use std::time::Instant;

struct Ix {
    a_offset_x: isize,
    a_offset_y: isize,
    b_offset_x: isize,
    b_offset_y: isize,
    prize_x: isize,
    prize_y: isize,
}

fn resolve(ix: Ix) -> usize {
    let mut amount_a = 0;
    let mut amount_b = 0;

    let mut current_x = 0;
    let mut current_y = 0;

    while ix.prize_x != current_x || ix.prize_y != current_y {
        while ix.prize_x > current_x + ix.b_offset_x || ix.prize_y > current_y + ix.b_offset_y {
            current_x += ix.b_offset_x;
            current_y += ix.b_offset_y;
            amount_b += 1;
        }

        if ix.prize_x == current_x && ix.prize_y == current_y {
            break;
        }

        current_x += ix.a_offset_x;
        current_y += ix.a_offset_y;
        amount_a += 1;
        while amount_b > 0 && (ix.prize_x < current_x || ix.prize_y < current_y) {
            current_x -= ix.b_offset_x;
            current_y -= ix.b_offset_y;
            amount_b -= 1;
        }
        if amount_b == 0 {
            break;
        }
    }

    if current_x == ix.prize_x && current_y == ix.prize_y {
        amount_a * 3 + amount_b
    } else {
        0
    }
}

// some boring math formula :(
fn resolve_q2(ix: Ix) -> usize {
    let det = ix.a_offset_x * ix.b_offset_y - ix.a_offset_y * ix.b_offset_x;
    let a = (ix.prize_x * ix.b_offset_y - ix.prize_y * ix.b_offset_x) / det;
    let b = (ix.prize_y * ix.a_offset_x - ix.prize_x * ix.a_offset_y) / det;

    if ix.a_offset_x * a + ix.b_offset_x * b == ix.prize_x
        && ix.a_offset_y * a + ix.b_offset_y * b == ix.prize_y
    {
        (a * 3 + b) as usize
    } else {
        0
    }
}

fn q1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|machine| {
            let mut iter = machine.lines();

            let (a_offset_x, a_offset_y) = iter
                .next()
                .unwrap()
                .trim_start_matches("Button A: X+")
                .split_once(", Y+")
                .map(|(x, y)| (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap()))
                .unwrap();

            let (b_offset_x, b_offset_y) = iter
                .next()
                .unwrap()
                .trim_start_matches("Button B: X+")
                .split_once(", Y+")
                .map(|(x, y)| (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap()))
                .unwrap();

            let (prize_x, prize_y) = iter
                .next()
                .unwrap()
                .trim_start_matches("Prize: X=")
                .split_once(", Y=")
                .map(|(x, y)| (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap()))
                .unwrap();

            Ix {
                a_offset_x,
                a_offset_y,
                b_offset_x,
                b_offset_y,
                prize_x,
                prize_y,
            }
        })
        .map(resolve)
        .sum::<usize>()
}

fn q2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|machine| {
            let mut iter = machine.lines();

            let (a_offset_x, a_offset_y) = iter
                .next()
                .unwrap()
                .trim_start_matches("Button A: X+")
                .split_once(", Y+")
                .map(|(x, y)| (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap()))
                .unwrap();

            let (b_offset_x, b_offset_y) = iter
                .next()
                .unwrap()
                .trim_start_matches("Button B: X+")
                .split_once(", Y+")
                .map(|(x, y)| (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap()))
                .unwrap();

            let (prize_x, prize_y) = iter
                .next()
                .unwrap()
                .trim_start_matches("Prize: X=")
                .split_once(", Y=")
                .map(|(x, y)| (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap()))
                .unwrap();

            Ix {
                a_offset_x,
                a_offset_y,
                b_offset_x,
                b_offset_y,
                prize_x: prize_x + 10000000000000,
                prize_y: prize_y + 10000000000000,
            }
        })
        .map(resolve_q2)
        .sum::<usize>()
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

    assert_eq!(q1(test), 480);
    assert_eq!(q2(test), 0);
}
