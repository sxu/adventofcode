use std::fs::File;
use std::io::{self, BufRead};

pub fn day3(input_path: &str) {
    let file =
        File::open(input_path).unwrap_or_else(|e| panic!("Failed to open {}: {}", input_path, e));
    let map: Vec<Vec<bool>> = io::BufReader::new(file)
        .lines()
        .map(|l| parse_row(&l.unwrap()))
        .collect();
    let width = map[0].len();
    for row in map.iter() {
        assert_eq!(row.len(), width);
    }
    assert_eq!(traverse(&map, width, 3, 1), 244);
    let product = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(right, down)| traverse(&map, width, right, down))
        .fold(1, |acc, x| acc * x);
    assert_eq!(product, 9406609920);
}

fn parse_row(row: &str) -> Vec<bool> {
    row.chars().map(|x| x == '#').collect()
}

fn traverse(map: &Vec<Vec<bool>>, width: usize, right: usize, down: usize) -> usize {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut count: usize = 0;
    while y < map.len() {
        if map[y][x] {
            count += 1;
        }
        x = (x + right) % width;
        y += down;
    }
    count
}
