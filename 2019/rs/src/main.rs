mod day1;
mod day19;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day9;
mod intcode;

use std::env;

fn stub(_input_path: &str) {}

fn main() {
    let days = [
        day1::day1,
        day2::day2,
        day3::day3,
        day4::day4,
        day5::day5,
        day6::day6,
        day7::day7,
        stub, // day8::day8,
        day9::day9,
        stub, // day10::day10,
        stub, // day11::day11,
        stub, // day12::day12,
        stub, // day13::day13,
        stub, // day14::day14,
        stub, // day15::day15,
        stub, // day16::day16,
        stub, // day17::day17,
        stub, // day18::day18,
        day19::day19,
    ];
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let day = args[1].parse::<usize>().unwrap();
        print!("Day {}..", day);
        days[day - 1](&format!("../input{}", day)[..]);
        println!(" OK");
    } else {
        for (i, day) in days.iter().enumerate() {
            print!("Day {}..", i + 1);
            day(&format!("../input{}", i + 1)[..]);
            println!(" OK");
        }
    }
}
