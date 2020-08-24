use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};

pub fn day1(input_path: &str) {
    let file =
        File::open(input_path).unwrap_or_else(|e| panic!("Failed to open {}: {}", input_path, e));
    let weights = io::BufReader::new(file)
        .lines()
        .map(|l| {
            l.unwrap_or_else(|e| panic!("Failed to read line: {}", e))
                .trim()
                .parse::<i32>()
                .unwrap_or_else(|e| panic!("Failed to parse integer {}", e))
        })
        .collect::<Vec<i32>>();
    assert_eq!(
        weights.iter().copied().map(calc_fuel_simple).sum::<i32>(),
        3455717
    );
    assert_eq!(
        weights.iter().copied().map(calc_fuel_complex).sum::<i32>(),
        5180690
    );
}

fn calc_fuel_simple(weight: i32) -> i32 {
    cmp::max(weight / 3 - 2, 0)
}

fn calc_fuel_complex(mut weight: i32) -> i32 {
    let mut sum: i32 = 0;
    while weight > 0 {
        let fuel = calc_fuel_simple(weight);
        sum += fuel;
        weight = fuel;
    }
    sum
}
