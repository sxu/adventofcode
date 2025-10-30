fn char_to_idx(c: char) -> u32 {
    c as u32 - 'a' as u32
}

fn indx_to_char(i: u32) -> char {
    (b'a' + i as u8) as char
}

fn check_password(password: &[u32]) -> bool {
    let mut has_increasing_subseq = false;
    for i in 0..password.len() - 2 {
        if password[i] + 1 == password[i + 1] && password[i] + 2 == password[i + 2] {
            has_increasing_subseq = true;
        }
    }
    if !has_increasing_subseq {
        return false;
    }

    let i = char_to_idx('i');
    let l = char_to_idx('l');
    let o = char_to_idx('o');
    for x in password.iter() {
        if *x == i || *x == l || *x == o {
            return false;
        }
    }

    let mut found_pairs = std::collections::HashSet::new();
    let mut i = 0;
    while i < password.len() - 1 {
        if password[i] == password[i + 1] {
            found_pairs.insert(password[i]);
            i += 2;
        } else {
            i += 1;
        }
    }
    if found_pairs.len() < 2 {
        return false;
    }

    true
}

fn step(password: &mut [u32]) {
    for i in (0..password.len()).rev() {
        password[i] += 1;
        if password[i] < 26 {
            break;
        }
        password[i] = 0;
    }
}

pub fn solve(input_path: &str) {
    let input = std::fs::read_to_string(input_path)
        .expect("Failed to read input")
        .trim()
        .to_owned();
    let mut password: Vec<u32> = input.chars().map(char_to_idx).collect();
    loop {
        step(&mut password);
        if check_password(&password) {
            let result: String = password.iter().map(|i| indx_to_char(*i)).collect();
            assert!(result == "cqjxxyzz");
            break;
        }
    }
    loop {
        step(&mut password);
        if check_password(&password) {
            let result: String = password.iter().map(|i| indx_to_char(*i)).collect();
            assert!(result == "cqkaabcc");
            break;
        }
    }
}
