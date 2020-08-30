use crate::intcode;
use std::cmp;
use std::collections::HashMap;
use std::collections::VecDeque;

const BLACK: i64 = 0;
const WHITE: i64 = 1;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use Direction::*;

fn turn_direction(cur: Direction, turn: i64) -> Direction {
    match turn {
        0 => turn_left(cur),
        1 => turn_right(cur),
        _ => panic!("Unexpected turn value {}", turn),
    }
}

fn turn_left(cur: Direction) -> Direction {
    match cur {
        Up => Left,
        Left => Down,
        Down => Right,
        Right => Up,
    }
}

fn turn_right(cur: Direction) -> Direction {
    match cur {
        Up => Right,
        Right => Down,
        Down => Left,
        Left => Up,
    }
}

fn move_forward(x: isize, y: isize, dir: Direction) -> (isize, isize) {
    match dir {
        Up => (x, y - 1),
        Down => (x, y + 1),
        Left => (x - 1, y),
        Right => (x + 1, y),
    }
}

fn paint_panels(rom: &Vec<i64>, starting_panel: i64) -> HashMap<(isize, isize), i64> {
    let mut computer = intcode::Computer::new(rom.clone());
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut dir = Up;
    let mut panels = HashMap::<(isize, isize), i64>::new();
    let mut inputs = VecDeque::from(vec![starting_panel]);
    computer.run_with(&mut inputs);
    while computer.state == intcode::State::WaitingForInput {
        let color = computer.outputs.pop_front().unwrap();
        let turn = computer.outputs.pop_front().unwrap();
        panels.insert((x, y), color);
        dir = turn_direction(dir, turn);
        let new = move_forward(x, y, dir);
        x = new.0;
        y = new.1;
        match panels.get(&(x, y)) {
            Some(color) => inputs.push_back(*color),
            None => inputs.push_back(BLACK),
        }
        computer.run_with(&mut inputs);
    }
    return panels;
}

pub fn day11(input_path: &str) {
    let rom = intcode::load_program(input_path);

    let panels = paint_panels(&rom, BLACK);
    assert_eq!(panels.len(), 1967);

    let panels = paint_panels(&rom, WHITE);
    let mut white_panels = panels
        .iter()
        .filter_map(|(&k, &v)| if v == WHITE { Some(k) } else { None })
        .collect::<Vec<(isize, isize)>>();
    let (min_x, min_y) = white_panels
        .iter()
        .fold((isize::MAX, isize::MAX), |p1, p2| {
            (cmp::min(p1.0, p2.0), cmp::min(p1.1, p2.1))
        });
    for coord in white_panels.iter_mut() {
        coord.0 -= min_x;
        coord.1 -= min_y;
    }
    let (max_x, max_y) = white_panels
        .iter()
        .fold((isize::MIN, isize::MIN), |p1, p2| {
            (cmp::max(p1.0, p2.0), cmp::max(p1.1, p2.1))
        });
    let width = (max_x + 1) as usize;
    let height = (max_y + 1) as usize;
    let mut render: Vec<i64> = Vec::new();
    render.resize(width * height, BLACK);
    for coord in white_panels.iter() {
        render[(coord.1 * width as isize + coord.0) as usize] = WHITE;
    }
    let render = render
        .iter()
        .map(|&x| if x == WHITE { '#' } else { ' ' })
        .collect::<String>();
    println!("");
    for i in (0..render.len()).step_by(width) {
        println!("{}", &render[i..(i + width)]);
    }
}
