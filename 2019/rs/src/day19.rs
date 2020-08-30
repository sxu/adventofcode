use crate::intcode;
use std::cmp;
use std::collections::VecDeque;

pub fn day19(input_path: &str) {
    let rom = intcode::load_program(input_path);

    let mut count = 0;
    for y in 0..50 {
        for x in 0..50 {
            let mut computer = intcode::Computer::new(rom.clone());
            let mut inputs = [x, y].iter().copied().collect::<VecDeque<i64>>();
            assert_eq!(computer.run_with(&mut inputs), intcode::State::Halted);
            assert_eq!(computer.outputs.len(), 1);
            if computer.outputs[0] == 1 {
                count += 1;
            }
        }
    }
    assert_eq!(count, 152);

    let mut search_region: VecDeque<Option<(usize, usize)>> = VecDeque::with_capacity(100);
    for y in 0..100 {
        let search_start = search_region.back().map_or(0, |x| x.map_or(0, |x| x.0));
        search_region.push_back(find_x_range(rom.clone(), y, search_start));
    }

    let mut closest = None;
    for y in 0..10000 {
        let mut maybe_width = Some((usize::MIN, usize::MAX));
        for maybe_range in search_region.iter() {
            match maybe_range {
                Some((new_start, new_end)) => {
                    maybe_width = maybe_width
                        .map(|(start, end)| (cmp::max(start, *new_start), cmp::min(end, *new_end)));
                }
                None => maybe_width = None,
            }
        }
        if let Some((start, end)) = maybe_width {
            if end > start && end - start >= 100 {
                closest = Some((start, y));
                break;
            }
        }
        search_region.pop_front();
        let search_start = search_region.back().unwrap().map_or(0, |x| x.0);
        search_region.push_back(find_x_range(rom.clone(), y + 100, search_start));
    }
    assert_eq!(closest, Some((1073, 411)));
}

fn find_x_range(rom: Vec<i64>, y: usize, x_start: usize) -> Option<(usize, usize)> {
    // TODO: use binary search/doubling stride
    let mut iter = (x_start..10000)
        .map(|x| {
            let mut computer = intcode::Computer::new(rom.clone());
            let mut inputs = [x, y]
                .iter()
                .copied()
                .map(|v| v as i64)
                .collect::<VecDeque<i64>>();
            assert_eq!(computer.run_with(&mut inputs), intcode::State::Halted);
            assert_eq!(computer.outputs.len(), 1);
            computer.outputs[0]
        })
        .enumerate()
        .skip_while(|(_, x)| *x == 0);

    if let Some((start, 1)) = iter.next() {
        if let Some((end, 0)) = iter.skip_while(|(_, x)| *x == 1).next() {
            Some((start + x_start as usize, end + x_start as usize))
        } else {
            panic!("Unreachable")
        }
    } else {
        None
    }
}
