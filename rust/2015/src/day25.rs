use chumsky::prelude::*;

fn parser<'a>() -> impl Parser<'a, &'a str, (usize, usize), extra::Err<Rich<'a, char>>> {
    let int = text::int(10).from_str().unwrapped();
    just("To continue, please consult the code grid in the manual.  Enter the code at row ")
        .ignore_then(int)
        .then_ignore(just(", column "))
        .then(int)
        .then_ignore(just(".\n").then(end()))
}

pub fn solve(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Failed to read file");
    let (row, col) = parser()
        .parse(&input)
        .into_result()
        .expect("Failed to parse input");
    let diagonal_start_row = row + col - 1;
    let pos_in_seq = diagonal_start_row * (diagonal_start_row - 1) / 2 + col;
    let mut code: usize = 20151125;
    for _ in 1..pos_in_seq {
        code = (code * 252533) % 33554393;
    }
    assert!(code == 2650453);
}
