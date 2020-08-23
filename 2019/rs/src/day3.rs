use itertools::Itertools;
use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};

pub fn day3(input_path: &str) {
    let file =
        File::open(input_path).unwrap_or_else(|e| panic!("Failed to open {}: {}", input_path, e));
    let wires: Vec<Vec<Segment>> = io::BufReader::new(file)
        .lines()
        .map(|line| {
            let line = line.unwrap_or_else(|e| panic!("Failed to read line: {}", e));
            let pieces = line.trim().split(",").map(parse_piece);
            follow_pieces(pieces)
        })
        .collect();
    assert_eq!(wires.len(), 2);

    let costs: Vec<(i32, i32)> = wires[0]
        .iter()
        .cartesian_product(wires[1].iter())
        .filter_map(|(s1, s2)| intersection_cost(s1, s2))
        .collect();
    let part1 = costs
        .iter()
        .map(|(x, _)| x)
        .min()
        .unwrap_or_else(|| panic!("Empty"));
    let part2 = costs
        .iter()
        .map(|(_, y)| y)
        .min()
        .unwrap_or_else(|| panic!("Empty"));
    assert_eq!(*part1, 870);
    assert_eq!(*part2, 13698);
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
struct Piece {
    pub dir: Direction,
    pub len: i32,
}

#[derive(Clone, Debug)]
struct Point(i32, i32);

#[derive(Debug)]
struct Segment {
    pub start: Point,
    pub end: Point,
    pub orientation: Orientation,
    pub steps_from_origin: i32,
}

fn parse_piece(input: &str) -> Piece {
    let dir = match input.chars().nth(0).unwrap_or_else(|| panic!("{}", input)) {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!("Unreachable"),
    };
    let len: i32 = input[1..].parse().unwrap_or_else(|e| panic!("{}", e));
    Piece { dir, len }
}

impl Piece {
    fn len(&self) -> i32 {
        self.len
    }

    fn endpoint(&self, start: &Point) -> Point {
        match self.dir {
            Direction::Up => Point(start.0, start.1 + self.len),
            Direction::Down => Point(start.0, start.1 - self.len),
            Direction::Left => Point(start.0 - self.len, start.1),
            Direction::Right => Point(start.0 + self.len, start.1),
        }
    }
}

impl Segment {
    fn new(start: &Point, piece: &Piece, steps_from_origin: i32) -> Segment {
        let end = piece.endpoint(start);
        let orientation = match piece.dir {
            Direction::Up => Orientation::Vertical,
            Direction::Down => Orientation::Vertical,
            Direction::Left => Orientation::Horizontal,
            Direction::Right => Orientation::Horizontal,
        };
        Segment {
            start: start.clone(),
            end,
            orientation,
            steps_from_origin,
        }
    }
}

fn follow_pieces<I>(pieces: I) -> Vec<Segment>
where
    I: Iterator<Item = Piece>,
{
    let mut wire: Vec<Segment> = Vec::new();
    let mut steps = 0;
    let mut start = Point(0, 0);
    for piece in pieces {
        let seg = Segment::new(&start, &piece, steps);
        start = seg.end.clone();
        steps += piece.len();
        wire.push(seg);
    }
    wire
}

fn intersection_cost(seg1: &Segment, seg2: &Segment) -> Option<(i32, i32)> {
    let mut hseg = seg1;
    let mut vseg = seg2;
    match (&seg1.orientation, &seg2.orientation) {
        (Orientation::Horizontal, Orientation::Horizontal) => return None,
        (Orientation::Vertical, Orientation::Vertical) => return None,
        (Orientation::Horizontal, Orientation::Vertical) => {}
        (Orientation::Vertical, Orientation::Horizontal) => {
            hseg = seg2;
            vseg = seg1;
        }
    }

    let x = vseg.start.0;
    let y = hseg.start.1;
    let left = cmp::min(hseg.start.0, hseg.end.0);
    let right = cmp::max(hseg.start.0, hseg.end.0);
    let top = cmp::max(vseg.start.1, vseg.end.1);
    let bottom = cmp::min(vseg.start.1, vseg.end.1);
    if (x == 0 && y == 0) || x < left || x > right || y < bottom || y > top {
        return None;
    }
    let extra1 = (x - seg1.start.0).abs() + (y - seg1.start.1).abs();
    let extra2 = (x - seg2.start.0).abs() + (y - seg2.start.1).abs();
    Some((
        x.abs() + y.abs(),
        seg1.steps_from_origin + seg2.steps_from_origin + extra1 + extra2,
    ))
}
