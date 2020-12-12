use std::collections::HashMap;
use std::collections::VecDeque;
use std::iter::FromIterator;

use crate::utils;

#[derive(Debug, Default)]
struct Joltages {
    list: VecDeque<i64>,
    val_to_count: HashMap<i64, usize>,
    sum: i64,
}

impl Joltages {
    fn new() -> Joltages {
        Default::default()
    }

    fn push_back(&mut self, x: i64) {
        self.list.push_back(x);
        *(self.val_to_count.entry(x).or_insert(0)) += 1;
        self.sum += x;
    }

    fn pop_front(&mut self) -> Option<i64> {
        let front = self.list.pop_front();
        if let Some(x) = front {
            let count = self.val_to_count.get_mut(&x).unwrap();
            if *count > 1 {
                *count -= 1;
            } else {
                self.val_to_count.remove(&x);
            }
            self.sum -= x;
        }
        front
    }

    fn accept(&self, x: i64) -> bool {
        for y in self.list.iter() {
            if self.val_to_count.contains_key(&(x - y)) {
                return true;
            }
        }
        return false;
    }

    fn sum(&self) -> i64 {
        self.sum
    }

    fn min(&self) -> Option<i64> {
        self.list.iter().cloned().min()
    }

    fn max(&self) -> Option<i64> {
        self.list.iter().cloned().max()
    }
}

impl FromIterator<i64> for Joltages {
    fn from_iter<I: IntoIterator<Item = i64>>(iter: I) -> Self {
        let mut ret = Joltages::new();
        for i in iter {
            ret.push_back(i);
        }
        ret
    }
}

pub fn day9(input_path: &str) {
    let inputs: Vec<i64> = utils::input_lines(input_path)
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut iter = inputs.iter().cloned();
    let mut jolts: Joltages = iter.by_ref().take(25).collect();
    let mut invalid: Option<i64> = None;
    for x in iter {
        if !jolts.accept(x) {
            invalid = Some(x);
            break;
        }
        jolts.pop_front();
        jolts.push_back(x);
    }
    let invalid = invalid;
    assert_eq!(invalid, Some(32321523));

    let mut weakness: Option<i64> = None;
    let mut jolts = Joltages::new();
    for x in inputs.iter().cloned() {
        jolts.push_back(x);
        while jolts.sum() > invalid.unwrap() {
            jolts.pop_front();
        }
        if jolts.sum() == invalid.unwrap() {
            weakness = Some(jolts.min().unwrap() + jolts.max().unwrap());
            break;
        }
    }
    assert_eq!(weakness, Some(4794981));
}
