use std::collections::HashSet;

use crate::utils;

pub fn day6(input_path: &str) {
    let lines = utils::input_lines(input_path);
    let mut unions: Vec<HashSet<char>> = Vec::new();
    let mut intersections: Vec<HashSet<char>> = Vec::new();
    let mut group: Vec<HashSet<char>> = Vec::new();

    unions.push(HashSet::new());
    for line in lines {
        if line.is_empty() {
            unions.push(HashSet::new());
            intersections.push(group.iter().fold(group.first().unwrap().clone(), |x, y| {
                x.intersection(y).cloned().collect::<HashSet<char>>()
            }));
            group = Vec::new();
        } else {
            let mut answer: HashSet<char> = HashSet::new();
            for c in line.chars() {
                unions.last_mut().unwrap().insert(c);
                answer.insert(c);
            }
            group.push(answer);
        }
    }
    intersections.push(group.iter().fold(group.first().unwrap().clone(), |x, y| {
        x.intersection(y).cloned().collect::<HashSet<char>>()
    }));

    assert_eq!(unions.iter().map(|a| a.len()).sum::<usize>(), 6161);
    assert_eq!(intersections.iter().map(|a| a.len()).sum::<usize>(), 2971);
}
