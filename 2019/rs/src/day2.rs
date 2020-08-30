use crate::intcode;

pub fn day2(input_path: &str) {
    let rom = intcode::load_program(input_path);

    let mut ram = rom.clone();
    ram[1] = 12;
    ram[2] = 2;
    let mut computer = intcode::Computer::new(ram);
    computer.run();
    assert_eq!(computer.ram[0], 5290681);

    let mut noun = None::<i64>;
    let mut verb = None::<i64>;
    for n in 0..100 {
        for v in 0..100 {
            let mut ram = rom.clone();
            ram[1] = n;
            ram[2] = v;
            let mut computer = intcode::Computer::new(ram);
            computer.run();
            if computer.ram[0] == 19690720 {
                noun = Some(n);
                verb = Some(v);
            }
        }
    }
    assert_eq!(noun, Some(57));
    assert_eq!(verb, Some(41));
}
