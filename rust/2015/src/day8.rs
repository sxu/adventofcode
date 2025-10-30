fn count_chars(s: &str) -> usize {
    let mut count = 0;
    let mut iter = s.chars();
    loop {
        let Some(c) = iter.next() else { break };
        if c == '\\' {
            const MSG: &str = "Incomplete escape sequence";
            let d = iter.next().expect(MSG);
            if d == 'x' {
                iter.next().expect(MSG);
                iter.next().expect(MSG);
            }
        }
        count += 1
    }
    count
}

fn count_escaped(s: &str) -> usize {
    let mut count = 0;
    for c in s.chars() {
        if c == '\\' || c == '\'' || c == '"' {
            count += 1;
        }
        count += 1;
    }
    count
}

pub fn solve(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Failed to read input");
    let mut string_size: usize = 0;
    let mut binary_size: usize = 0;
    let mut escaped_size: usize = 0;
    for l in input.lines() {
        string_size += l.len();
        binary_size += count_chars(&l[1..l.len() - 1]);
        escaped_size += count_escaped(l) + 2;
    }
    assert!(string_size - binary_size == 1371);
    assert!(escaped_size - string_size == 2117);
}
