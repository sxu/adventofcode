mod day1;
mod day2;

use std::env;

fn main() {
    let days = [day1::day1, day2::day2];
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
