use crate::lights::Grid;

fn count_on_neighbors(grid: &Grid<bool>, i: isize, j: isize) -> u32 {
    let at = |ii: isize, jj: isize| {
        if ii < 0 || jj < 0 || ii >= grid.height() as isize || jj >= grid.width() as isize {
            false
        } else {
            *grid.at(ii as usize, jj as usize)
        }
    };

    let mut count = 0;
    for ii in (i - 1)..=(i + 1) {
        for jj in (j - 1)..=(j + 1) {
            count += at(ii, jj) as u32
        }
    }
    count - at(i, j) as u32
}

fn turn_on_corners(grid: &mut Grid<bool>) {
    for (i, j) in [
        (0, 0),
        (0, grid.width() - 1),
        (grid.height() - 1, 0),
        (grid.height() - 1, grid.width() - 1),
    ] {
        *grid.at_mut(i, j) = true;
    }
}

fn step(grid: &Grid<bool>) -> Grid<bool> {
    let mut new = Grid::<bool>::new(grid.height(), grid.width(), false);
    for i in 0..grid.height() {
        for j in 0..grid.width() {
            let count = count_on_neighbors(grid, i as isize, j as isize);
            if *grid.at(i, j) {
                if count == 2 || count == 3 {
                    *new.at_mut(i, j) = true;
                }
            } else if count == 3 {
                *new.at_mut(i, j) = true;
            }
        }
    }
    new
}

pub fn solve(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Failed to read file.");
    let from_input = || {
        let mut grid = Grid::<bool>::new(100, 100, false);
        for (i, line) in input.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                *grid.at_mut(i, j) = c == '#';
            }
        }
        grid
    };

    let mut grid = from_input();
    for _ in 0..100 {
        grid = step(&grid);
    }
    let n_on = grid.data().iter().fold(0, |acc, x| acc + *x as usize);
    assert!(n_on == 1061);

    let mut grid = from_input();
    turn_on_corners(&mut grid);
    for _ in 0..100 {
        grid = step(&grid);
        turn_on_corners(&mut grid);
    }
    let n_on = grid.data().iter().fold(0, |acc, x| acc + *x as usize);
    assert!(n_on == 1006);
}
