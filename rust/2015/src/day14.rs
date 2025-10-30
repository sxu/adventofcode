use chumsky::prelude::*;

#[derive(PartialEq)]
enum State {
    Flying,
    Resting,
}

struct Reindeer {
    speed: u32,
    fly_time: u32,
    rest_time: u32,
    distance: u32,
    state: State,
    time_left: u32,
}

impl Reindeer {
    fn new(speed: u32, fly_time: u32, rest_time: u32) -> Self {
        Reindeer {
            speed,
            fly_time,
            rest_time,
            distance: 0,
            state: State::Flying,
            time_left: fly_time,
        }
    }

    fn reset(&mut self) {
        self.distance = 0;
        self.state = State::Flying;
        self.time_left = self.fly_time;
    }

    fn go(&mut self, mut time: u32) -> u32 {
        while time > 0 {
            let step = std::cmp::min(self.time_left, time);
            if self.state == State::Flying {
                self.distance += self.speed * step;
            }
            time -= step;
            self.time_left -= step;
            if self.time_left == 0 {
                if self.state == State::Flying {
                    self.state = State::Resting;
                    self.time_left = self.rest_time;
                } else {
                    self.state = State::Flying;
                    self.time_left = self.fly_time;
                }
            }
        }
        self.distance
    }
}

fn parser<'a>() -> impl Parser<'a, &'a str, Vec<Reindeer>, extra::Err<Rich<'a, char>>> {
    let spaces = just(" ").repeated();
    let reindeer = text::ident()
        .ignored()
        .then_ignore(just("can fly").padded_by(spaces))
        .then(text::int(10).from_str::<u32>().unwrapped())
        .then_ignore(just("km/s for").padded_by(spaces))
        .then(text::int(10).from_str::<u32>().unwrapped())
        .then_ignore(just("seconds, but then must rest for").padded_by(spaces))
        .then(text::int(10).from_str::<u32>().unwrapped())
        .then_ignore(just("seconds.").padded_by(spaces))
        .map(|(((_, speed), fly_time), rest_time)| Reindeer::new(speed, fly_time, rest_time));
    reindeer
        .separated_by(text::newline())
        .allow_trailing()
        .collect()
        .then_ignore(end())
}

fn race_reindeers(reindeers: &mut [Reindeer], total_time: u32) -> u32 {
    for r in reindeers.iter_mut() {
        r.reset();
    }

    let mut scores: Vec<u32> = vec![0; reindeers.len()];
    for _ in 0..total_time {
        let mut max: u32 = 0;
        for r in reindeers.iter_mut() {
            max = std::cmp::max(r.go(1), max);
        }
        for (i, r) in reindeers.iter().enumerate() {
            if r.distance == max {
                scores[i] += 1;
            }
        }
    }
    *scores.iter().max().unwrap()
}

pub fn solve(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Failed to read input");
    let mut reindeers = parser()
        .parse(&input)
        .into_result()
        .expect("Failed to parse input");
    let max_dist = reindeers.iter_mut().map(|reindeer| reindeer.go(2503)).max();
    assert!(max_dist == Some(2655));
    assert!(race_reindeers(&mut reindeers, 2503) == 1059);
}
