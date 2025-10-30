fn find_combinations<F>(target: u32, containers: &[u32], n_containers_used: usize, cb: &mut F)
where
    F: FnMut(usize),
{
    if target == 0 {
        cb(n_containers_used);
        return;
    }
    if containers.is_empty() {
        return;
    }

    let size = containers[0];
    if size <= target {
        find_combinations(target - size, &containers[1..], n_containers_used + 1, cb);
    }
    find_combinations(target, &containers[1..], n_containers_used, cb);
}

pub fn solve(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Failed to read input");
    let containers: Vec<u32> = input.lines().map(|l| l.parse::<u32>().unwrap()).collect();
    let mut count = 0;
    find_combinations(150, &containers, 0, &mut |_| count += 1);
    assert!(count == 1638);

    count = 0;
    let mut min_used = None;
    find_combinations(150, &containers, 0, &mut |n_used| match min_used {
        None => {
            min_used = Some(n_used);
            count = 1;
        }
        Some(n) if n > n_used => {
            min_used = Some(n_used);
            count = 1;
        }
        Some(n) if n == n_used => count += 1,
        _ => (),
    });
    assert!(count == 17);
}
