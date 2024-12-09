use std::time::Instant;

fn q1(input: &str) -> usize {
    let decompressed_data = input
        .chars()
        .enumerate()
        .filter_map(|(idx, c)| c.to_digit(10).map(|digit| (idx, digit as usize)))
        .map(|(idx, digit)| vec![if idx % 2 == 0 { Some(idx / 2) } else { None }; digit])
        .flatten()
        .collect::<Vec<_>>();

    let mut low = 0;
    let mut high = decompressed_data.len() - 1;

    let mut res = 0;
    while low <= high {
        let number = match decompressed_data[low] {
            Some(number) => number * low,
            None => {
                while decompressed_data[high].is_none() {
                    high -= 1;
                }

                let number = decompressed_data[high].unwrap();
                high -= 1;
                number * low
            }
        };

        res += number;
        low += 1;
    }

    res
}

enum Content {
    File { id: usize, space: usize },
    Free { space: usize },
}

fn q2(input: &str) -> usize {
    let mut max_idx = 0;

    let mut data = input
        .chars()
        .enumerate()
        .filter_map(|(idx, c)| c.to_digit(10).map(|digit| (idx, digit as usize)))
        .filter(|(_, c)| *c != 0)
        .map(|(idx, digit)| {
            if idx % 2 == 0 {
                let idx = idx / 2;
                max_idx = max_idx.max(idx);
                Content::File {
                    id: idx,
                    space: digit,
                }
            } else {
                Content::Free { space: digit }
            }
        })
        .collect::<Vec<_>>();

    for id_to_move in (0..=max_idx).rev() {
        let Some((idx_to_move, space_to_move)) =
            data.iter()
                .enumerate()
                .find_map(|(idx, space)| match space {
                    Content::File { id, space } if *id == id_to_move => Some((idx, *space)),
                    _ => None,
                })
        else {
            // id has len of 0 so doesn't exist
            continue;
        };

        for i in 0..idx_to_move {
            if let Content::Free { space } = &mut data[i] {
                if *space >= space_to_move {
                    *space -= space_to_move;

                    data[idx_to_move] = Content::Free {
                        space: space_to_move,
                    };
                    data.insert(
                        i,
                        Content::File {
                            id: id_to_move,
                            space: space_to_move,
                        },
                    );

                    break;
                }
            }
        }
    }
    let mut res = 0;
    let mut idx = 0;
    for space in data {
        match space {
            Content::File { id, space } => {
                for _ in 0..space {
                    res += id * idx;

                    idx += 1;
                }
            }
            Content::Free { space } => idx += space,
        }
    }

    res
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

    assert_eq!(q1(test), 1928);
    assert_eq!(q2(test), 2858);
}
