use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::FromIterator;

pub fn day1(input_path: &str) {
    let file =
        File::open(input_path).unwrap_or_else(|e| panic!("Failed to open {}: {}", input_path, e));
    let xs = io::BufReader::new(file)
        .lines()
        .map(|l| {
            l.unwrap_or_else(|e| panic!("Failed to read line: {}", e))
                .trim()
                .parse::<i32>()
                .unwrap_or_else(|e| panic!("Failed to parse integer {}", e))
        })
        .collect::<Vec<i32>>();
    let set: HashSet<i32> = HashSet::from_iter(xs.iter().cloned());

    assert_eq!(find_product2(2020, &xs, &set), Some(437931));
    assert_eq!(find_product3(2020, &xs, &set), Some(157667328));
}

pub fn find_product2(target: i32, xs: &[i32], set: &HashSet<i32>) -> Option<i32> {
    for x in xs {
        let y = target - x;
        if set.contains(&y) {
            return Some(x * y);
        }
    }
    None
}

pub fn find_product3(target: i32, xs: &[i32], set: &HashSet<i32>) -> Option<i32> {
    for i in 0..xs.len() {
        let target2 = target - xs[i];
        if let Some(y) = find_product2(target2, &xs[(i + 1)..], set) {
            return Some(xs[i] * y);
        }
    }
    None
}
