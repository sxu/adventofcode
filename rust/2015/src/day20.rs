fn find_first_house(
    target: usize,
    presents_per_house: usize,
    max_deliveries: Option<usize>,
) -> usize {
    let mut houses = vec![1; (target / presents_per_house) + 1];
    for i in 2..houses.len() {
        let mut deliveries = 0;
        for j in (i..houses.len()).step_by(i) {
            houses[j] += i * presents_per_house;
            if let Some(max) = max_deliveries {
                deliveries += 1;
                if deliveries >= max {
                    break;
                }
            }
        }
    }
    for (i, &n_presents) in houses.iter().enumerate() {
        if n_presents >= target {
            return i;
        }
    }
    unreachable!();
}

pub fn solve(input_path: &str) {
    let n = std::fs::read_to_string(input_path)
        .expect("Failed to read input")
        .trim()
        .parse::<usize>()
        .expect("Failed to parse input");
    assert!(find_first_house(n, 10, None) == 665280);
    assert!(find_first_house(n, 11, Some(50)) == 705600);
}
