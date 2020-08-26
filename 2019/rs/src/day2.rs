use crate::intcode;

pub fn day2(input_path: &str) {
    let rom = std::fs::read_to_string(input_path)
        .unwrap_or_else(|e| panic!("Failed to open {}: {}", input_path, e))
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut ram = rom.clone();
    ram[1] = 12;
    ram[2] = 2;
    intcode::run(&mut ram);
    assert_eq!(ram[0], 5290681);

    let mut noun = None::<i32>;
    let mut verb = None::<i32>;
    for n in 0..100 {
        for v in 0..100 {
            let mut ram = rom.clone();
            ram[1] = n;
            ram[2] = v;
            intcode::run(&mut ram);
            if ram[0] == 19690720 {
                noun = Some(n);
                verb = Some(v);
            }
        }
    }
    assert_eq!(noun, Some(57));
    assert_eq!(verb, Some(41));
}
