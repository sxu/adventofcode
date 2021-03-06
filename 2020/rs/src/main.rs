mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod utils;

use std::env;

fn main() {
    let days = [
        day1::day1,
        day2::day2,
        day3::day3,
        day4::day4,
        day5::day5,
        day6::day6,
        day7::day7,
        day8::day8,
        day9::day9,
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
