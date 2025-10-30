use itertools::Itertools;

fn partition(xs: &[usize], target1: usize, target2: usize) -> bool {
    let mut dp = vec![vec![false; target2 + 1]; target1 + 1];
    dp[0][0] = true;
    for &x in xs {
        for sum1 in 0..=target1 {
            for sum2 in 0..=target2 {
                if sum1 >= x {
                    dp[sum1][sum2] |= dp[sum1 - x][sum2];
                }
                if sum2 >= x {
                    dp[sum1][sum2] |= dp[sum1][sum2 - x];
                }
            }
        }
    }
    dp[target1][target2]
}

fn find_min_entropy(xs: &[usize], target: usize, extra_compartments: bool) -> usize {
    let extra_target = if extra_compartments { target } else { 0 };
    for i in 1..=xs.len() {
        let mut found = false;
        let mut min_entanglement = usize::MAX;
        for group in xs.iter().copied().combinations(i) {
            if group.iter().sum::<usize>() != target {
                continue;
            }

            let mut rest = Vec::<usize>::new();
            for x in xs.iter() {
                if !group.contains(x) {
                    rest.push(*x);
                }
            }
            if partition(&rest, target, extra_target) {
                found = true;
                min_entanglement = std::cmp::min(group.iter().product(), min_entanglement);
            }
        }
        if found {
            return min_entanglement;
        }
    }
    unreachable!();
}

pub fn solve(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Failed to read file");
    let mut xs = Vec::<usize>::new();
    for line in input.lines() {
        xs.push(line.parse::<usize>().unwrap());
    }
    let sum: usize = xs.iter().sum();
    assert!(sum % 3 == 0);
    assert!(sum % 4 == 0);

    assert!(find_min_entropy(&xs, sum / 3, false) == 10723906903);
    assert!(find_min_entropy(&xs, sum / 4, true) == 74850409);
}
