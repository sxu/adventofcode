use itertools::Itertools;

use crate::utils;

struct PolicyAndPassword {
    c: char,
    lower: usize,
    upper: usize,
    password: String,
}

fn parse_input(input: &str) -> PolicyAndPassword {
    let (policy, password) = input.split(": ").next_tuple().unwrap();
    let (range, c) = policy.split(' ').next_tuple().unwrap();
    let (lower, upper) = range
        .split('-')
        .map(|x| x.parse::<usize>().unwrap())
        .next_tuple()
        .unwrap();
    PolicyAndPassword {
        c: c.chars().nth(0).unwrap(),
        lower,
        upper,
        password: String::from(password),
    }
}

fn is_valid1(p: &PolicyAndPassword) -> bool {
    let count = p.password.matches(p.c).count();
    p.lower <= count && count <= p.upper
}

fn is_valid2(p: &PolicyAndPassword) -> bool {
    let a = p.password.chars().nth(p.lower - 1) == Some(p.c);
    let b = p.password.chars().nth(p.upper - 1) == Some(p.c);
    a != b
}

pub fn day2(input_path: &str) {
    let xs: Vec<PolicyAndPassword> = utils::input_lines(input_path)
        .map(|l| parse_input(&l))
        .collect();
    assert_eq!(xs.iter().filter(|x| is_valid1(x)).count(), 500);
    assert_eq!(xs.iter().filter(|x| is_valid2(x)).count(), 313);
}
