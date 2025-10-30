use crate::utils::for_each_permutation;
use chumsky::prelude::*;
use std::collections::HashMap;

fn parser<'a>()
-> impl Parser<'a, &'a str, Vec<((&'a str, &'a str), usize)>, extra::Err<Simple<'a, char>>> {
    let city = text::ident();
    let trip = city
        .then_ignore(just("to").padded())
        .then(city)
        .then_ignore(just("=").padded())
        .then(text::int(10).from_str::<usize>().unwrapped());
    trip.separated_by(text::newline())
        .allow_trailing()
        .collect()
        .then_ignore(end())
}

fn tsp(graph: &[Vec<usize>]) -> (usize, usize) {
    let n = graph.len();
    let mut min = usize::MAX;
    let mut max = 0;
    let mut eval = |permutation: &[usize]| {
        let mut dist = 0;
        for i in 0..(permutation.len() - 1) {
            dist += graph[permutation[i]][permutation[i + 1]];
        }
        if dist < min {
            min = dist;
        }
        if dist > max {
            max = dist;
        }
    };
    let mut permutation: Vec<usize> = (0..n).collect();
    for_each_permutation(&mut permutation, &mut eval);
    (min, max)
}

pub fn solve(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Failed to read input");
    let edges = parser()
        .parse(&input)
        .into_result()
        .expect("Failed to parse input");
    let mut city_to_id = HashMap::<&str, usize>::new();
    let mut next_id = 0;
    for ((c1, c2), _) in edges.iter() {
        city_to_id.entry(c1).or_insert_with(|| {
            let id = next_id;
            next_id += 1;
            id
        });
        city_to_id.entry(c2).or_insert_with(|| {
            let id = next_id;
            next_id += 1;
            id
        });
    }
    let n = city_to_id.len();
    let mut graph: Vec<Vec<usize>> = vec![vec![0; n]; n];
    for ((c1, c2), dist) in edges.iter() {
        let v1 = city_to_id[c1];
        let v2 = city_to_id[c2];
        graph[v1][v2] = *dist;
        graph[v2][v1] = *dist;
    }
    let (min, max) = tsp(&graph);
    assert!(min == 117);
    assert!(max == 909);
}
