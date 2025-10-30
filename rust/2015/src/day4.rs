fn mine_coin(key: &str, target_leading_zeros: usize) -> u32 {
    let prefix = "0".repeat(target_leading_zeros);
    let mut x: u32 = 0;
    loop {
        let digest = md5::compute(format!("{key}{x}"));
        let hex = format!("{digest:x}");
        if hex.starts_with(&prefix) {
            break x;
        }
        x += 1;
    }
}

pub fn solve(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Failed to read file.");
    let key = input.trim();

    assert_eq!(mine_coin(key, 5), 282749);
    assert_eq!(mine_coin(key, 6), 9962624);
}
