use itertools::Itertools;
use std::collections::HashMap;

#[derive(Default)]
struct Santa {
    x: i32,
    y: i32,
}

impl Santa {
    fn step(&mut self, dir: char, deliveries: &mut HashMap<(i32, i32), u32>) {
        (self.x, self.y) = match dir {
            '^' => (self.x, self.y + 1),
            'v' => (self.x, self.y - 1),
            '>' => (self.x + 1, self.y),
            '<' => (self.x - 1, self.y),
            _ => panic!("Unexpected direction {dir}"),
        };
        *deliveries.entry((self.x, self.y)).or_insert(0) += 1;
    }

    fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
    }
}

pub fn solve(input_path: &str) {
    let directions = std::fs::read_to_string(input_path).expect("Failed to read file.");
    let mut santa = Santa::default();
    let mut deliveries: HashMap<(i32, i32), u32> = HashMap::new();
    deliveries.insert((0, 0), 1);
    for dir in directions.trim().chars() {
        santa.step(dir, &mut deliveries);
    }
    assert_eq!(deliveries.len(), 2592);

    deliveries.clear();
    deliveries.insert((0, 0), 2);
    santa.reset();
    let mut robot_santa = Santa::default();
    let mut iter = directions.trim().chars();
    while let Some((dir1, dir2)) = iter.next_tuple() {
        santa.step(dir1, &mut deliveries);
        robot_santa.step(dir2, &mut deliveries);
    }
    if let Some(dir) = iter.next() {
        santa.step(dir, &mut deliveries);
    }
    assert_eq!(deliveries.len(), 2360);
}
