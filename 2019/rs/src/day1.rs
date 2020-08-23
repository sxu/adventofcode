use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};

pub fn day1(input_path: &str) {
    let file = File::open(input_path).expect("Failed to open input file");
    let weights = io::BufReader::new(file)
        .lines()
        .map(|l| {
            l.expect("Failed to read line")
                .trim()
                .parse::<i32>()
                .expect("Failed to parse weight")
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
