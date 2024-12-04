enum Direction {
    Up,
    RightUp,
    Right,
    DownRight,
    Down,
    LeftDown,
    Left,
    LeftUp,
}

const ALL_DIRECTIONS: [Direction; 8] = [
    Direction::Up,
    Direction::RightUp,
    Direction::Right,
    Direction::DownRight,
    Direction::Down,
    Direction::LeftDown,
    Direction::Left,
    Direction::LeftUp,
];

impl Direction {
    fn is_xmas(&self, grid: &[Vec<char>], x: usize, y: usize, last_char: Option<char>) -> bool {
        if let Some(&current_char) = grid.get(x).map(|line| line.get(y)).flatten() {
            match (last_char, current_char) {
                // This is ok, we search next
                (None, 'X') | (Some('X'), 'M') | (Some('M'), 'A') => {}
                // It's XMAS !
                (Some('A'), 'S') => return true,
                _ => return false,
            }

            if let Some((new_x, new_y)) = self.next_pos(x, y) {
                return self.is_xmas(grid, new_x, new_y, Some(current_char));
            } else {
                return false;
            }
        }

        return false;
    }

    fn next_pos(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        let (x_offset, y_offset) = self.offset();

        Some((
            x.checked_add_signed(x_offset)?,
            y.checked_add_signed(y_offset)?,
        ))
    }

    fn offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (1, 0),
            Direction::RightUp => (1, 1),
            Direction::Right => (0, 1),
            Direction::DownRight => (-1, 1),
            Direction::Down => (-1, 0),
            Direction::LeftDown => (-1, -1),
            Direction::Left => (0, -1),
            Direction::LeftUp => (1, -1),
        }
    }
}

fn q1(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut res = 0;

    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            res += ALL_DIRECTIONS
                .iter()
                .filter(|direction| direction.is_xmas(&grid, x, y, None))
                .count()
        }
    }

    res
}

fn is_x_mas(grid: &[Vec<char>], x: usize, y: usize) -> Option<()> {
    let current_char = grid.get(x).map(|line| line.get(y)).flatten().copied()?;

    if current_char != 'A' {
        return None;
    }

    let (left_up_x, left_up_y) = Direction::LeftUp.next_pos(x, y)?;
    let left_up_char = grid
        .get(left_up_x)
        .map(|line| line.get(left_up_y))
        .flatten()
        .copied()?;

    let (left_down_x, left_down_y) = Direction::LeftDown.next_pos(x, y)?;
    let left_down_char = grid
        .get(left_down_x)
        .map(|line| line.get(left_down_y))
        .flatten()
        .copied()?;

    let (right_up_x, right_up_y) = Direction::RightUp.next_pos(x, y)?;
    let right_up_char = grid
        .get(right_up_x)
        .map(|line| line.get(right_up_y))
        .flatten()
        .copied()?;

    let (down_right_x, down_right_y) = Direction::DownRight.next_pos(x, y)?;
    let down_right_char = grid
        .get(down_right_x)
        .map(|line| line.get(down_right_y))
        .flatten()
        .copied()?;

    if is_cross_mas(right_up_char, left_down_char) && is_cross_mas(down_right_char, left_up_char) {
        return Some(());
    } else {
        return None;
    }
}

fn is_cross_mas(up: char, down: char) -> bool {
    (up == 'M' && down == 'S') || (down == 'M' && up == 'S')
}

fn q2(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut res = 0;

    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if is_x_mas(&grid, x, y).is_some() {
                res += 1;
            }
        }
    }

    res
}

fn main() {
    let real = include_str!("../input.real.txt");
    dbg!(q1(real));
    dbg!(q2(real));
}

#[test]
fn test() {
    let test = include_str!("../input.test.txt");

    assert_eq!(q1(test), 18);
    assert_eq!(q2(test), 9);
}
