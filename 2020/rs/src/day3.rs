use crate::utils;

pub fn day3(input_path: &str) {
    let map: Vec<Vec<bool>> = utils::input_lines(input_path)
        .map(|l| parse_row(&l))
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
