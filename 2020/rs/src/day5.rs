use crate::utils;

struct Seat {
    row: u32,
    col: u32,
}

impl Seat {
    fn id(&self) -> u32 {
        self.row * 8 + self.col
    }
}

fn parse_seat(input: &str) -> Seat {
    let mut row: u32 = 0;
    for c in input[..7].chars() {
        row = row * 2 + if c == 'B' { 1 } else { 0 };
    }
    let mut col: u32 = 0;
    for c in input[7..].chars() {
        col = col * 2 + if c == 'R' { 1 } else { 0 };
    }
    Seat { row, col }
}

pub fn day5(input_path: &str) {
    let mut seat_ids: Vec<u32> = utils::input_lines(input_path)
        .map(|l| parse_seat(&l).id())
        .collect();
    seat_ids.sort();

    let first = *seat_ids.first().unwrap();
    let last = *seat_ids.last().unwrap();

    assert_eq!(last, 922);

    let sum: u32 = seat_ids.iter().sum();
    assert_eq!((first + last) * (last - first + 1) / 2 - sum, 747);
}
