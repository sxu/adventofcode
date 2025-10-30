fn look_and_say(input: &str) -> String {
    let mut output = String::new();
    let mut current: Option<char> = None;
    let mut cnt = 0;
    for c in input.chars() {
        if current == Some(c) {
            cnt += 1;
        } else {
            if let Some(x) = current {
                output += &(cnt.to_string() + &x.to_string());
            }
            current = Some(c);
            cnt = 1;
        }
    }
    output += &(cnt.to_string() + &current.unwrap().to_string());
    output
}

pub fn solve(input_path: &str) {
    let input = std::fs::read_to_string(input_path)
        .expect("Failed to read input")
        .trim()
        .to_owned();
    let mut sequence = input.clone();
    for _ in 0..40 {
        sequence = look_and_say(&sequence)
    }
    assert!(sequence.len() == 492982);
    for _ in 0..10 {
        sequence = look_and_say(&sequence)
    }
    assert!(sequence.len() == 6989950);
}
