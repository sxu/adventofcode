use chumsky::prelude::*;

#[derive(Debug)]
enum Instr {
    Half(usize),
    Triple(usize),
    Inc(usize),
    Jmp(i32),
    JmpIfEven(usize, i32),
    JmpIfOne(usize, i32),
}

fn parser<'a>() -> impl Parser<'a, &'a str, Vec<Instr>, extra::Err<Rich<'a, char>>> {
    let register = choice((just("a").to(0), just("b").to(1)));
    let offset = choice((just("+").to(1), just("-").to(-1)))
        .then(text::int(10).from_str::<i32>().unwrapped())
        .map(|(x, y)| x * y);
    let half = just("hlf ").ignore_then(register).map(Instr::Half);
    let triple = just("tpl ").ignore_then(register).map(Instr::Triple);
    let inc = just("inc ").ignore_then(register).map(Instr::Inc);
    let jmp = just("jmp ").ignore_then(offset).map(Instr::Jmp);
    let jie = just("jie ")
        .ignore_then(register)
        .then_ignore(just(", "))
        .then(offset)
        .map(|(r, o)| Instr::JmpIfEven(r, o));
    let jio = just("jio ")
        .ignore_then(register)
        .then_ignore(just(", "))
        .then(offset)
        .map(|(r, o)| Instr::JmpIfOne(r, o));
    let instr = choice((half, triple, inc, jmp, jie, jio));
    instr
        .separated_by(text::newline())
        .allow_trailing()
        .collect()
        .then_ignore(end())
}

fn run(program: &[Instr], a: u32, b: u32) -> u32 {
    let mut pc: i32 = 0;
    let mut registers: [u32; 2] = [a, b];
    while pc >= 0 && pc < program.len() as i32 {
        pc += 1;
        match program[(pc - 1) as usize] {
            Instr::Half(r) => {
                registers[r] /= 2;
            }
            Instr::Triple(r) => {
                registers[r] *= 3;
            }
            Instr::Inc(r) => {
                registers[r] += 1;
            }
            Instr::Jmp(o) => {
                pc += o - 1;
            }
            Instr::JmpIfEven(r, o) => {
                if registers[r] % 2 == 0 {
                    pc += o - 1;
                }
            }
            Instr::JmpIfOne(r, o) => {
                if registers[r] == 1 {
                    pc += o - 1;
                }
            }
        }
    }
    registers[1]
}

pub fn solve(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Failed to read file");
    let program = parser()
        .parse(&input)
        .into_result()
        .expect("Failed to parse input");
    assert!(run(&program, 0, 0) == 307);
    assert!(run(&program, 1, 0) == 160);
}
