use crate::utils::for_each_permutation;
use chumsky::prelude::*;
use std::collections::HashMap;

fn parser<'a>()
-> impl Parser<'a, &'a str, Vec<(&'a str, &'a str, isize)>, extra::Err<Simple<'a, char>>> {
    let gain = just("gain").padded().to(1);
    let lose = just("lose").padded().to(-1);
    let pairing = text::ident()
        .then_ignore(just("would").padded())
        .then(gain.or(lose))
        .then(text::int(10).from_str::<isize>().unwrapped())
        .then_ignore(just("happiness units by sitting next to").padded())
        .then(text::ident())
        .then_ignore(just("."))
        .map(|(((person, multiplier), happiness), other)| (person, other, happiness * multiplier));
    pairing
        .separated_by(text::newline())
        .allow_trailing()
        .collect()
        .then_ignore(end())
}

pub fn solve(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Failed to read input");
    let pairings = parser()
        .parse(&input)
        .into_result()
        .expect("Failed to parse input");
    let mut preferences = HashMap::<String, HashMap<String, isize>>::new();
    for (person, other, delta) in pairings {
        let pref = preferences.entry(person.to_string()).or_default();
        pref.insert(other.to_string(), delta);
    }

    fn compute_max_happiness(
        friends: &mut [String],
        preferences: &HashMap<String, HashMap<String, isize>>,
    ) -> isize {
        let mut max_happiness = isize::MIN;
        let mut eval = |arrangement: &[String]| {
            let n = arrangement.len();
            let mut total_happiness = 0;
            for i in 0..(n - 1) {
                total_happiness += preferences[&arrangement[i]]
                    .get(&arrangement[i + 1])
                    .unwrap_or(&0);
                total_happiness += preferences[&arrangement[i + 1]]
                    .get(&arrangement[i])
                    .unwrap_or(&0);
            }
            total_happiness += preferences[&arrangement[0]]
                .get(&arrangement[n - 1])
                .unwrap_or(&0);
            total_happiness += preferences[&arrangement[n - 1]]
                .get(&arrangement[0])
                .unwrap_or(&0);
            max_happiness = std::cmp::max(max_happiness, total_happiness);
        };
        for_each_permutation(friends, &mut eval);
        max_happiness
    }

    let mut friends: Vec<String> = preferences.keys().cloned().collect();
    let max_happiness = compute_max_happiness(&mut friends, &preferences);
    assert!(max_happiness == 709);

    friends.push("Me".to_string());
    preferences.entry("Me".to_string()).or_default();
    let max_happiness_with_me = compute_max_happiness(&mut friends, &preferences);
    assert!(max_happiness_with_me == 668);
}
