use itertools::Itertools;
use std::collections::HashMap;

use crate::utils;

type BaggingRules = HashMap<String, Vec<(u32, String)>>;

fn parse_rule(line: &str, rules: &mut BaggingRules) {
    let (outer_bag, contents) = line.split(" bags contain ").next_tuple().unwrap();
    for content in contents.split(", ") {
        if content == "no other bags." {
            continue;
        }
        let (count, rest) = content.splitn(2, ' ').next_tuple().unwrap();
        let count = count.parse::<u32>().unwrap();
        let (_, inner_bag) = rest.rsplitn(2, ' ').next_tuple().unwrap();
        rules
            .entry(outer_bag.to_string())
            .or_default()
            .push((count, inner_bag.trim().to_string()));
    }
}

fn can_contain(
    outer: &str,
    target: &str,
    rules: &BaggingRules,
    cache: &mut HashMap<String, bool>,
) -> bool {
    match cache.get(outer) {
        Some(result) => *result,
        None => {
            let mut result = false;
            for (_, inner) in rules.get(outer).unwrap_or(&vec![]) {
                if inner == target || can_contain(inner, target, rules, cache) {
                    result = true;
                    break;
                }
            }
            cache.insert(outer.to_string(), result);
            result
        }
    }
}

fn count_contained_bags(
    outer: &str,
    rules: &BaggingRules,
    cache: &mut HashMap<String, u32>,
) -> u32 {
    match cache.get(outer) {
        Some(result) => *result,
        None => {
            let mut total: u32 = 0;
            for (count, inner) in rules.get(outer).unwrap_or(&vec![]) {
                total += count + count * count_contained_bags(inner, rules, cache);
            }
            total
        }
    }
}

pub fn day7(input_path: &str) {
    let mut rules: BaggingRules = BaggingRules::new();
    utils::input_lines(input_path).for_each(|l| parse_rule(&l, &mut rules));

    let mut cache: HashMap<String, bool> = HashMap::new();
    assert_eq!(
        rules
            .keys()
            .filter(|k| can_contain(k, "shiny gold", &rules, &mut cache))
            .count(),
        211
    );

    let mut cache: HashMap<String, u32> = HashMap::new();
    assert_eq!(
        count_contained_bags("shiny gold", &rules, &mut cache),
        12414
    );
}
