use std::{fs::File, io::Write, time::Instant};

#[derive(Debug, Clone, Copy)]
struct Robot {
    speed_x: isize,
    speed_y: isize,
    has_moved: bool,
}

impl Robot {
    pub fn next_pos(
        &self,
        map: &[Vec<Vec<Robot>>],
        current_x: usize,
        current_y: usize,
    ) -> (usize, usize) {
        let mut next_x_signed = current_x as isize + (self.speed_x % map.len() as isize);
        let mut next_y_signed = current_y as isize + (self.speed_y % map[0].len() as isize);

        let width = map.len() as isize;

        if next_x_signed >= width {
            next_x_signed -= width;
        }
        if next_x_signed < 0 {
            next_x_signed += width;
        }

        let height = map[0].len() as isize;

        if next_y_signed >= height {
            next_y_signed -= height;
        }
        if next_y_signed < 0 {
            next_y_signed += height;
        }

        (next_x_signed as usize, next_y_signed as usize)
    }
}

fn q1(input: &str, w: usize, h: usize) -> usize {
    let mut map: Vec<Vec<Vec<Robot>>> = vec![vec![vec![]; h]; w];

    for line in input.lines() {
        let (position, speed) = line.split_once(' ').unwrap();
        let (position_x, position_y) = position
            .trim_start_matches("p=")
            .split_once(",")
            .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
            .unwrap();

        let (speed_x, speed_y) = speed
            .trim_start_matches("v=")
            .split_once(",")
            .map(|(x, y)| (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap()))
            .unwrap();

        map[position_x][position_y].push(Robot {
            speed_x,
            speed_y,
            has_moved: false,
        });
    }

    for _ in 0..100 {
        for x in 0..map.len() {
            for y in 0..map[x].len() {
                let robots = map[x][y].drain(..).collect::<Vec<_>>();

                for mut robot in robots {
                    if robot.has_moved {
                        map[x][y].push(robot);
                    } else {
                        let (new_x, new_y) = robot.next_pos(&map, x, y);

                        robot.has_moved = true;
                        map[new_x][new_y].push(robot);
                    }
                }
            }
        }
        for line in &mut map {
            for robots in line {
                for robot in robots {
                    robot.has_moved = false;
                }
            }
        }
    }

    let mut up_right_quadrants = 0;
    let mut up_left_quadrants = 0;
    let mut down_right_quadrants = 0;
    let mut down_left_quadrants = 0;

    let x_split = map.len() / 2;
    let y_split = map[0].len() / 2;
    for (x, line) in map.into_iter().enumerate() {
        for (y, robots) in line.into_iter().enumerate() {
            if x > x_split && y < y_split {
                up_right_quadrants += robots.len();
            } else if x < x_split && y < y_split {
                up_left_quadrants += robots.len();
            } else if x > x_split && y > y_split {
                down_right_quadrants += robots.len();
            } else if x < x_split && y > y_split {
                down_left_quadrants += robots.len();
            }
        }
    }

    up_right_quadrants * up_left_quadrants * down_right_quadrants * down_left_quadrants
}

fn q2(input: &str, w: usize, h: usize) -> usize {
    let mut map: Vec<Vec<Vec<Robot>>> = vec![vec![vec![]; h]; w];

    for line in input.lines() {
        let (position, speed) = line.split_once(' ').unwrap();
        let (position_x, position_y) = position
            .trim_start_matches("p=")
            .split_once(",")
            .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
            .unwrap();

        let (speed_x, speed_y) = speed
            .trim_start_matches("v=")
            .split_once(",")
            .map(|(x, y)| (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap()))
            .unwrap();

        map[position_x][position_y].push(Robot {
            speed_x,
            speed_y,
            has_moved: false,
        });
    }

    // maybe looking at the numbre of neighbors robots instead of thousands of image is better, but
    // yeah..
    for secs in 0..20000 {
        for x in 0..map.len() {
            for y in 0..map[x].len() {
                let robots = map[x][y].drain(..).collect::<Vec<_>>();

                for mut robot in robots {
                    if robot.has_moved {
                        map[x][y].push(robot);
                    } else {
                        let (new_x, new_y) = robot.next_pos(&map, x, y);

                        robot.has_moved = true;
                        map[new_x][new_y].push(robot);
                    }
                }
            }
        }
        for line in &mut map {
            for robots in line {
                for robot in robots {
                    robot.has_moved = false;
                }
            }
        }
        let filename = format!("robots/{secs}.ppm");

        let mut file = File::create(filename).unwrap();
        // Write the PPM header
        writeln!(file, "P6").unwrap(); // P6 indicates binary format
        writeln!(file, "{} {}", w, h).unwrap(); // Image dimensions
        writeln!(file, "255").unwrap(); // Max color value

        for y in 0..h {
            for x in 0..w {
                let r = 0 as u8;
                let g = if map[x][y].is_empty() { 0 } else { y as u8 };
                let b = 0 as u8;

                file.write_all(&[r, g, b]).unwrap(); // Write RGB values in binary
            }
        }
    }

    let mut up_right_quadrants = 0;
    let mut up_left_quadrants = 0;
    let mut down_right_quadrants = 0;
    let mut down_left_quadrants = 0;

    let x_split = map.len() / 2;
    let y_split = map[0].len() / 2;
    for (x, line) in map.into_iter().enumerate() {
        for (y, robots) in line.into_iter().enumerate() {
            if x > x_split && y < y_split {
                up_right_quadrants += robots.len();
            } else if x < x_split && y < y_split {
                up_left_quadrants += robots.len();
            } else if x > x_split && y > y_split {
                down_right_quadrants += robots.len();
            } else if x < x_split && y > y_split {
                down_left_quadrants += robots.len();
            }
        }
    }

    up_right_quadrants * up_left_quadrants * down_right_quadrants * down_left_quadrants
}

fn main() {
    let real = include_str!("../input.real.txt");
    let now = Instant::now();
    let sol_q1 = q1(real, 101, 103);
    println!("q1: {sol_q1} | {:?}", now.elapsed());
    let now = Instant::now();
    let sol_q2 = q2(real, 101, 103);
    println!("q2: {sol_q2} | {:?}", now.elapsed());
}

#[test]
fn test() {
    let test = include_str!("../input.test.txt");

    assert_eq!(q1(test, 11, 7), 12);
}
