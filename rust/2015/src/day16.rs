use chumsky::prelude::*;
use std::collections::HashMap;

fn parser<'a>() -> impl Parser<'a, &'a str, Vec<HashMap<&'a str, u32>>, extra::Err<Rich<'a, char>>>
{
    let preamble = just("Sue ")
        .then(text::int(10).from_str::<u32>())
        .then(just(": "));
    let property_name = just("children")
        .or(just("cats"))
        .or(just("samoyeds"))
        .or(just("pomeranians"))
        .or(just("akitas"))
        .or(just("vizslas"))
        .or(just("goldfish"))
        .or(just("trees"))
        .or(just("cars"))
        .or(just("perfumes"));
    let property = property_name
        .then_ignore(just(": "))
        .then(text::int(10).from_str::<u32>().unwrapped());
    let properties = property
        .separated_by(just(", "))
        .collect()
        .map(|ps: Vec<(&str, u32)>| HashMap::<&str, u32>::from_iter(ps));
    let sue = preamble.ignore_then(properties);
    sue.separated_by(text::newline())
        .allow_trailing()
        .collect()
        .then_ignore(end())
}

fn match1(sue: &HashMap<&str, u32>, detection: &HashMap<&str, u32>) -> bool {
    for (property, val) in detection {
        if let Some(expected) = sue.get(property) {
            if val != expected {
                return false;
            }
        }
    }
    true
}

fn match2(sue: &HashMap<&str, u32>, detection: &HashMap<&str, u32>) -> bool {
    for (property, val) in detection {
        if let Some(expected) = sue.get(property) {
            let pass = match *property {
                "cats" | "tree" => val < expected,
                "pomeranians" | "goldfish" => val > expected,
                _ => val == expected,
            };
            if !pass {
                return false;
            }
        }
    }
    true
}

pub fn solve(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Failed to read input");
    let sues = parser()
        .parse(&input)
        .into_result()
        .expect("Failed to parse input");
    let detection: HashMap<&str, u32> = HashMap::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);
    let mut gifter_sue1: usize = 0;
    let mut gifter_sue2: usize = 0;
    for (i, sue) in sues.iter().enumerate() {
        if match1(sue, &detection) {
            assert!(gifter_sue1 == 0);
            gifter_sue1 = i + 1;
        }
        if match2(sue, &detection) {
            assert!(gifter_sue2 == 0);
            gifter_sue2 = i + 1;
        }
    }
    assert!(gifter_sue1 == 40);
    assert!(gifter_sue2 == 241);
}
