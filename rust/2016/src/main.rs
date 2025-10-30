#![warn(clippy::all)]

mod day1;

use std::env;

fn main() {
    let solutions = [day1::solve];

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let day = args[1].parse::<usize>().expect("Failed to parse day");
        print!("Day {day}..");
        solutions[day - 1](&format!("../../inputs/2016/day{day}"));
        println!(" OK");
    } else {
        for (i, solve) in solutions.iter().enumerate() {
            print!("Day {}..", i + 1);
            solve(&format!("../../inputs/2016/day{}", i + 1));
            println!(" OK");
        }
    }
}
