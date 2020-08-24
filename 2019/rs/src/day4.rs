pub fn day4(input_path: &str) {
    let min_max: Vec<i32> = std::fs::read_to_string(input_path)
        .unwrap_or_else(|e| panic!("Failed to open {}: {}", input_path, e))
        .trim()
        .split("-")
        .map(|x| {
            x.parse::<i32>()
                .unwrap_or_else(|e| panic!("Failed to parse {}", e))
        })
        .collect();
    assert_eq!(min_max.len(), 2);
    let min = min_max[0];
    let max = min_max[1];

    let mut part1 = 0;
    let mut part2 = 0;
    let mut x = min;
    while x <= max {
        let digits = x
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        x += 1;

        if !is_non_decreasing(&digits[..]) {
            continue;
        }
        if has_same_adjacent_digits(&digits[..]) {
            part1 += 1;
            if has_same_adjacent_pair(&digits[..]) {
                part2 += 1;
            }
        }
    }
    assert_eq!(part1, 1955);
    assert_eq!(part2, 1319);
}

fn is_non_decreasing(digits: &[u32]) -> bool {
    let mut maybe_last = None::<u32>;
    for x in digits.iter() {
        if let Some(last) = maybe_last {
            if last > *x {
                return false;
            }
        }
        maybe_last = Some(*x);
    }
    true
}

fn has_same_adjacent_digits(digits: &[u32]) -> bool {
    let mut maybe_last = None::<u32>;
    for x in digits.iter() {
        if let Some(last) = maybe_last {
            if last == *x {
                return true;
            }
        }
        maybe_last = Some(*x);
    }
    false
}

fn has_same_adjacent_pair(mut digits: &[u32]) -> bool {
    while !digits.is_empty() {
        if let [_] = digits {
            return false;
        }
        if let [x, y] = digits {
            return x == y;
        }
        if let [x, y, z] = digits[..3] {
            if x == y {
                if x != z {
                    return true;
                }
                let mut i = 3;
                while i < digits.len() {
                    if digits[i] != x {
                        break;
                    }
                    i += 1;
                }
                digits = &digits[i..];
            } else {
                digits = &digits[1..];
            }
        } else {
            panic!("Unreachable");
        }
    }
    false
}
