use std::collections::{HashMap, HashSet};

use chumsky::prelude::*;
use itertools::Itertools;

fn input_parser<'a>()
-> impl Parser<'a, &'a str, (Vec<(&'a str, &'a str)>, &'a str), extra::Err<Rich<'a, char>>> {
    let molecule = text::ident();
    let replacement = molecule.then_ignore(just(" => ")).then(molecule);
    let replacements = replacement.separated_by(text::newline()).collect();
    replacements
        .then_ignore(text::newline())
        .then_ignore(text::newline())
        .then(molecule)
        .then_ignore(text::newline())
        .then_ignore(end())
}

fn calibrate(replacements: &Vec<(&str, &str)>, target: &str) -> usize {
    let mut new_molecules = HashSet::<String>::new();
    for (old, new) in replacements {
        for (i, _) in target.match_indices(old) {
            let replaced = target[..i].to_string() + new + &target[(i + old.len())..];
            new_molecules.insert(replaced.to_owned());
        }
    }
    new_molecules.len()
}

fn element_parser<'a>() -> impl Parser<'a, &'a str, &'a str, extra::Err<Rich<'a, char>>> {
    let uppercase = any().filter(|c: &char| c.is_uppercase());
    let lowercase = any().filter(|c: &char| c.is_lowercase());
    uppercase.then(lowercase.or_not()).to_slice()
}

fn molecule_parser<'a>() -> impl Parser<'a, &'a str, Vec<&'a str>, extra::Err<Rich<'a, char>>> {
    element_parser().repeated().collect::<Vec<&str>>()
}

#[derive(Debug)]
enum Rule<T: Eq> {
    Terminal(T, T),
    NonTerminal(T, T, T),
}

impl<T: Eq> Rule<T> {
    fn terminal(from: T, to: T) -> Self {
        Rule::Terminal(from, to)
    }

    fn non_terminal(from: T, to1: T, to2: T) -> Self {
        Rule::NonTerminal(from, to1, to2)
    }

    fn from(&self) -> &T {
        match self {
            Rule::Terminal(from, _) => from,
            Rule::NonTerminal(from, _, _) => from,
        }
    }
}

fn create_grammar<'a>(
    all_elements: &HashSet<&'a str>,
    replacements: &Vec<(&'a str, &'a str)>,
) -> Vec<(Rule<&'a str>, usize)> {
    let mut grammar = Vec::<(Rule<&str>, usize)>::new();
    for el in all_elements {
        grammar.push((Rule::terminal(el, el), 0));
    }

    let first_parser = element_parser()
        .then(any().repeated().to_slice())
        .then_ignore(end());
    for replacement in replacements {
        let mut from = replacement.0;
        let mut to = replacement.1;
        let mut weight = 1;
        loop {
            let (first, rest) = first_parser.parse(to).into_result().unwrap();
            if rest.is_empty() {
                break;
            }
            grammar.push((Rule::non_terminal(from, first, rest), weight));
            from = rest;
            to = rest;
            weight = 0;
        }
    }
    grammar
}

fn cyk_parse(input: &Vec<&str>, rules: &Vec<(Rule<&str>, usize)>) -> Vec<Vec<Vec<Option<usize>>>> {
    let mut rule_indices = HashMap::<&str, Vec<usize>>::new();
    for (i, (rule, _)) in rules.iter().enumerate() {
        rule_indices.entry(rule.from()).or_default().push(i);
    }

    let n = input.len();
    let r = rules.len();
    // parse_tree[l][s][r]
    // - l: length of input span - 1
    // - s: start index of input span
    // - r: rule index
    let mut parse_table = vec![vec![vec![None::<usize>; r]; n]; n];
    for (i, x) in input.iter().enumerate() {
        for (j, (rule, _)) in rules.iter().enumerate() {
            match rule {
                Rule::Terminal(_, y) if x == y => parse_table[0][i][j] = Some(0),
                _ => (),
            }
        }
    }

    for l in 2..=n {
        for s in 0..=(n - l) {
            for (r, (rule, weight)) in rules.iter().enumerate() {
                if let Rule::NonTerminal(_, to1, to2) = rule {
                    let indices1 = rule_indices.get(to1).unwrap();
                    let indices2 = rule_indices.get(to2).unwrap();
                    let mut min = None;
                    for p in 1..l {
                        for (i1, i2) in indices1.iter().cartesian_product(indices2.iter()) {
                            if let (Some(x1), Some(x2)) = (
                                parse_table[p - 1][s][*i1],
                                parse_table[l - p - 1][s + p][*i2],
                            ) {
                                if let Some(x) = min {
                                    min = Some(std::cmp::min(x, x1 + x2 + *weight));
                                } else {
                                    min = Some(x1 + x2 + weight);
                                }
                            }
                        }
                    }
                    parse_table[l - 1][s][r] = min;
                }
            }
        }
    }
    parse_table
}

pub fn solve(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Failed to read input");
    let (replacements, target) = input_parser()
        .parse(&input)
        .into_result()
        .expect("Failed to parse input");
    assert!(calibrate(&replacements, target) == 509);

    let target = molecule_parser()
        .then_ignore(end())
        .parse(target)
        .into_result()
        .expect("Failed to parse target molecule");
    let all_elements = target.iter().cloned().collect::<HashSet<&str>>();
    let grammar = create_grammar(&all_elements, &replacements);
    let parse_table = cyk_parse(&target, &grammar);
    assert!(*parse_table[target.len() - 1][0].last().unwrap() == Some(195));
}
