use chumsky::prelude::*;

fn parser<'a>() -> impl Parser<'a, &'a str, Vec<Vec<i32>>, extra::Err<Rich<'a, char>>> {
    let value = just("-")
        .or_not()
        .then(text::int(10).from_str::<i32>())
        .map(|(maybe_neg, val)| {
            if maybe_neg.is_some() {
                -val.unwrap()
            } else {
                val.unwrap()
            }
        });
    let property = just("capacity ")
        .or(just("durability "))
        .or(just("flavor "))
        .or(just("texture "))
        .or(just("calories "));
    let ingredient = (text::ident().then(just(": "))).ignore_then(
        property
            .ignore_then(value)
            .separated_by(just(", "))
            .collect(),
    );
    ingredient
        .separated_by(text::newline())
        .allow_trailing()
        .collect()
        .then_ignore(end())
}

fn for_each_combination<F>(n: usize, total: i32, cb: &mut F)
where
    F: FnMut(&[i32]),
{
    let mut combination = Vec::<i32>::new();
    fn rec<F>(i: usize, combo: &mut Vec<i32>, n: usize, total: i32, cb: &mut F)
    where
        F: FnMut(&[i32]),
    {
        let used: i32 = combo.iter().sum();
        if i == n - 1 {
            let last = total - used;
            combo.push(last);
            cb(combo);
            combo.pop();
        } else {
            for new in 0..(total - used) {
                combo.push(new);
                rec(i + 1, combo, n, total, cb);
                combo.pop();
            }
        }
    }
    rec(0, &mut combination, n, total, cb);
}

pub fn solve(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Failed to read input");
    let ingredients = parser()
        .parse(&input)
        .into_result()
        .expect("Failed to parse input");
    assert!(ingredients.len() == 4);
    let mut max_score: i32 = 0;
    let mut max_500_cal_score: i32 = 0;
    let mut evaluate = |combination: &[i32]| {
        assert!(combination.len() == ingredients.len());
        let mut score: i32 = 1;
        for property in 0..4 {
            let property_score = ingredients
                .iter()
                .enumerate()
                .fold(0, |acc, (idx, x)| acc + x[property] * combination[idx]);
            score *= std::cmp::max(property_score, 0);
        }
        max_score = std::cmp::max(score, max_score);
        let calories: i32 = ingredients
            .iter()
            .enumerate()
            .fold(0, |acc, (idx, x)| acc + x[4] * combination[idx]);
        if calories == 500 {
            max_500_cal_score = std::cmp::max(score, max_500_cal_score);
        }
    };
    for_each_combination(ingredients.len(), 100, &mut evaluate);
    assert!(max_score == 18965440);
    assert!(max_500_cal_score == 15862900);
}
