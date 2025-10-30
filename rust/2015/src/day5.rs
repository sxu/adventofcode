use std::collections::HashMap;

trait StreamChecker {
    fn process(&mut self, c: char);
    fn check(&self) -> bool;
    fn reset(&mut self);
}

struct AtLeastThreeVowelsChecker {
    cnt: usize,
}

impl AtLeastThreeVowelsChecker {
    fn new() -> Self {
        Self { cnt: 0 }
    }
}

impl StreamChecker for AtLeastThreeVowelsChecker {
    fn process(&mut self, c: char) {
        self.cnt += match c {
            'a' | 'e' | 'i' | 'o' | 'u' => 1,
            _ => 0,
        };
    }

    fn check(&self) -> bool {
        self.cnt >= 3
    }

    fn reset(&mut self) {
        self.cnt = 0;
    }
}

struct RepeatedLetterChecker {
    last_c: Option<char>,
    detected: bool,
}

impl RepeatedLetterChecker {
    fn new() -> Self {
        Self {
            last_c: None,
            detected: false,
        }
    }
}

impl StreamChecker for RepeatedLetterChecker {
    fn process(&mut self, c: char) {
        if self.check() {
            return;
        }
        if let Some(last) = self.last_c
            && last == c
        {
            self.detected = true;
        }
        self.last_c = Some(c);
    }

    fn check(&self) -> bool {
        self.detected
    }

    fn reset(&mut self) {
        self.last_c = None;
        self.detected = false;
    }
}

struct ForbiddenPairsChecker {
    last_c: Option<char>,
    detected: bool,
}

impl ForbiddenPairsChecker {
    fn new() -> Self {
        Self {
            last_c: None,
            detected: false,
        }
    }
}

impl StreamChecker for ForbiddenPairsChecker {
    fn process(&mut self, c: char) {
        self.detected |= match self.last_c {
            Some('a') => c == 'b',
            Some('c') => c == 'd',
            Some('p') => c == 'q',
            Some('x') => c == 'y',
            _ => false,
        };
        self.last_c = Some(c);
    }

    fn check(&self) -> bool {
        !self.detected
    }

    fn reset(&mut self) {
        self.last_c = None;
        self.detected = false;
    }
}

struct NonOverlappingReoccuringPairChecker {
    pos: usize,
    last_c: Option<char>,
    pair_to_pos: HashMap<(char, char), usize>,
    detected: bool,
}

impl NonOverlappingReoccuringPairChecker {
    fn new() -> Self {
        Self {
            pos: 0,
            last_c: None,
            pair_to_pos: HashMap::new(),
            detected: false,
        }
    }
}

impl StreamChecker for NonOverlappingReoccuringPairChecker {
    fn process(&mut self, c: char) {
        if self.check() {
            return;
        }
        if let Some(last) = self.last_c {
            if let Some(old_pos) = self.pair_to_pos.get(&(last, c)) {
                self.detected |= self.pos - old_pos > 1;
            } else {
                self.pair_to_pos.insert((last, c), self.pos);
            }
        }
        self.last_c = Some(c);
        self.pos += 1;
    }

    fn check(&self) -> bool {
        self.detected
    }

    fn reset(&mut self) {
        self.pos = 0;
        self.last_c = None;
        self.pair_to_pos.clear();
        self.detected = false;
    }
}

struct RepeatingGappedLetterChecker {
    last_2_c: [char; 2],
    detected: bool,
}

impl RepeatingGappedLetterChecker {
    fn new() -> Self {
        Self {
            last_2_c: ['\0', '\0'],
            detected: false,
        }
    }
}

impl StreamChecker for RepeatingGappedLetterChecker {
    fn process(&mut self, c: char) {
        if self.last_2_c[0] == c {
            self.detected = true
        }
        self.last_2_c[0] = self.last_2_c[1];
        self.last_2_c[1] = c;
    }

    fn check(&self) -> bool {
        self.detected
    }

    fn reset(&mut self) {
        self.last_2_c = ['\0', '\0'];
        self.detected = false;
    }
}

fn count_nice(input: &str, checkers: &mut [Box<dyn StreamChecker>]) -> usize {
    let mut nice_cnt: usize = 0;
    for line in input.lines() {
        for checker in checkers.iter_mut() {
            checker.reset();
        }
        for c in line.chars() {
            for checker in checkers.iter_mut() {
                checker.process(c);
            }
        }
        if checkers.iter().all(|checker| checker.check()) {
            nice_cnt += 1;
        }
    }
    nice_cnt
}

pub fn solve(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Failed to read file.");

    let mut checkers: Vec<Box<dyn StreamChecker>> = Vec::new();
    checkers.push(Box::new(AtLeastThreeVowelsChecker::new()));
    checkers.push(Box::new(RepeatedLetterChecker::new()));
    checkers.push(Box::new(ForbiddenPairsChecker::new()));
    assert_eq!(count_nice(input.as_str(), &mut checkers), 258);

    checkers.clear();
    checkers.push(Box::new(NonOverlappingReoccuringPairChecker::new()));
    checkers.push(Box::new(RepeatingGappedLetterChecker::new()));
    assert_eq!(count_nice(input.as_str(), &mut checkers), 53);
}
