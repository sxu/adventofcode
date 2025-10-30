use std::collections::HashSet;

enum Turn {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
enum Direction {
    East = 0,
    South = 1,
    West = 2,
    North = 3,
}

impl Direction {
    fn from_int(x: u32) -> Self {
        match x % 4 {
            0 => Self::East,
            1 => Self::South,
            2 => Self::West,
            3 => Self::North,
            _ => unreachable!(),
        }
    }

    fn turn(&self, turn: Turn) -> Self {
        let ival = *self as u32 + 4;
        let ival = match turn {
            Turn::Left => ival - 1,
            Turn::Right => ival + 1,
        };
        Self::from_int(ival)
    }
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Position {
    fn walk(&mut self, dist: i32) {
        let (x, y) = match self.direction {
            Direction::East => (self.x + dist, self.y),
            Direction::South => (self.x, self.y - dist),
            Direction::West => (self.x - dist, self.y),
            Direction::North => (self.x, self.y + dist),
        };
        self.x = x;
        self.y = y;
    }
}

pub fn solve(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Failed to read file");
    let dirs = input.trim().split(", ").collect::<Vec<&str>>();
    let mut pos = Position {
        x: 0,
        y: 0,
        direction: Direction::North,
    };
    let mut first_revisited_loc: Option<(i32, i32)> = None;
    let mut visited = HashSet::<(i32, i32)>::new();
    visited.insert((pos.x, pos.y));
    for dir in dirs {
        let dist = dir[1..].parse::<i32>().unwrap();
        let turn = match dir.chars().next().unwrap() {
            'L' => Turn::Left,
            'R' => Turn::Right,
            _ => unreachable!(),
        };
        pos.direction = pos.direction.turn(turn);
        for _ in 0..dist {
            pos.walk(1);
            if visited.contains(&(pos.x, pos.y)) {
                if first_revisited_loc.is_none() {
                    first_revisited_loc = Some((pos.x, pos.y));
                }
            } else {
                visited.insert((pos.x, pos.y));
            }
        }
    }
    assert!(pos.x.abs() + pos.y.abs() == 250);
    let first_revisited_loc = first_revisited_loc.unwrap();
    assert!(first_revisited_loc.0.abs() + first_revisited_loc.1.abs() == 151);
}
