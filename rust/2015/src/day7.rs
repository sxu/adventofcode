use chumsky::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
enum Operand {
    Literal(u16),
    Wire(String),
}

#[derive(Debug)]
enum Expr {
    Value(Operand),
    Not(Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    LShift(Operand, Operand),
    RShift(Operand, Operand),
}

#[derive(Debug)]
struct Instr {
    expr: Expr,
    wire: String,
}

fn parser<'a>() -> impl Parser<'a, &'a str, Vec<Instr>, extra::Err<Simple<'a, char>>> {
    let ident = text::ident().map(|s: &str| s.to_string());

    let operand = text::int(10)
        .from_str()
        .unwrapped()
        .map(Operand::Literal)
        .or(ident.map(Operand::Wire))
        .padded();

    let not = just("NOT").padded().ignore_then(operand).map(Expr::Not);

    let binary = operand
        .then(
            choice((
                just("AND").to(Expr::And as fn(_, _) -> Expr),
                just("OR").to(Expr::Or as fn(_, _) -> Expr),
                just("LSHIFT").to(Expr::LShift as fn(_, _) -> Expr),
                just("RSHIFT").to(Expr::RShift as fn(_, _) -> Expr),
            ))
            .padded(),
        )
        .then(operand)
        .map(|((x, op), y)| op(x, y));

    let expr = not.or(binary).or(operand.map(Expr::Value));

    let instr = expr
        .then_ignore(just("->").padded())
        .then(ident)
        .map(|(expr, wire)| Instr { expr, wire });

    instr
        .separated_by(text::newline())
        .allow_trailing()
        .collect::<Vec<Instr>>()
        .then_ignore(end())
}

#[derive(Default)]
struct Evaluator<'a> {
    expr_map: HashMap<&'a str, &'a Expr>,
    value_map: HashMap<&'a str, u16>,
}

impl<'a> Evaluator<'a> {
    fn eval_operand(&mut self, operand: &'a Operand) -> u16 {
        match operand {
            Operand::Literal(v) => *v,
            Operand::Wire(wire) => {
                if let Some(v) = self.value_map.get(&wire[..]) {
                    *v
                } else {
                    self.eval_wire(wire)
                }
            }
        }
    }

    fn eval_expr(&mut self, expr: &'a Expr) -> u16 {
        match expr {
            Expr::Value(op) => self.eval_operand(op),
            Expr::Not(op) => !self.eval_operand(op),
            Expr::And(op1, op2) => self.eval_operand(op1) & self.eval_operand(op2),
            Expr::Or(op1, op2) => self.eval_operand(op1) | self.eval_operand(op2),
            Expr::LShift(op1, op2) => self.eval_operand(op1) << self.eval_operand(op2),
            Expr::RShift(op1, op2) => self.eval_operand(op1) >> self.eval_operand(op2),
        }
    }

    fn eval_wire(&mut self, wire: &'a str) -> u16 {
        if let Some(v) = self.value_map.get(wire) {
            *v
        } else {
            let expr = self.expr_map.get(wire).expect("Undefined wire");
            let v = self.eval_expr(expr);
            self.value_map.insert(wire, v);
            v
        }
    }

    fn add_instruction(&mut self, wire: &'a str, expr: &'a Expr) {
        self.expr_map.insert(wire, expr);
    }
}

pub fn solve(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Failed to read input");
    let instructions = parser()
        .parse(&input)
        .into_result()
        .expect("Failed to parse input");
    let mut evaluator = Evaluator::default();
    for instr in instructions.iter() {
        evaluator.add_instruction(&instr.wire, &instr.expr);
    }
    let a = evaluator.eval_wire("a");
    assert!(a == 956);

    evaluator.value_map = vec![("b", a)].into_iter().collect();
    let a = evaluator.eval_wire("a");
    assert!(a == 40149);
}
