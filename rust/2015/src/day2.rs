pub fn solve(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Failed to read file.");
    let extract_dims = |desc: &str| {
        let mut dims: Vec<i32> = desc
            .split('x')
            .map(|dim| dim.parse::<i32>().unwrap())
            .collect();
        assert_eq!(dims.len(), 3);
        dims.sort();
        dims
    };
    let boxes: Vec<Vec<i32>> = input.lines().map(extract_dims).collect();
    let mut total_wrapper_area: i32 = 0;
    let mut total_ribbon_length: i32 = 0;
    for dims in boxes {
        let a1 = dims[0] * dims[1];
        let a2 = dims[1] * dims[2];
        let a3 = dims[2] * dims[0];
        total_wrapper_area += 2 * (a1 + a2 + a3) + dims[0] * dims[1];
        total_ribbon_length += 2 * (dims[0] + dims[1]) + dims[0] * dims[1] * dims[2];
    }
    assert_eq!(total_wrapper_area, 1588178);
    assert_eq!(total_ribbon_length, 3783758);
}
