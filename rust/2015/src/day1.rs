pub fn solve(input_path: &str) {
    let directions = std::fs::read_to_string(input_path).expect("Failed to read file.");
    let mut floor: i64 = 0;
    let mut basement_time: Option<usize> = None;
    for (i, c) in directions.trim().chars().enumerate() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        } else {
            panic!("Unexpected character {c}");
        }
        if floor == -1 && basement_time.is_none() {
            let _ = basement_time.insert(i + 1);
        }
    }
    assert_eq!(floor, 138);
    assert_eq!(basement_time.expect("Never reached basement"), 1771);
}
