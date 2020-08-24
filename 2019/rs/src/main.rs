mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let days = [day1::day1, day2::day2, day3::day3, day4::day4];
    for (i, day) in days.iter().enumerate() {
        print!("Day {}..", i + 1);
        day(&format!("../input{}", i + 1)[..]);
        println!(" OK");
    }
}
