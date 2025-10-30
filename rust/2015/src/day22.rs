use chumsky::prelude::*;
use either::*;

fn parser<'a>() -> impl Parser<'a, &'a str, (i32, i32), extra::Err<Rich<'a, char>>> {
    group((
        just("Hit Points: ")
            .ignore_then(text::int(10).from_str::<i32>().unwrapped())
            .then_ignore(text::newline()),
        just("Damage: ")
            .ignore_then(text::int(10).from_str::<i32>().unwrapped())
            .then_ignore(text::newline()),
    ))
    .then_ignore(end())
}

#[derive(Clone, Copy, Debug)]
struct State {
    hp: i32,
    mp: i32,
    boss_hp: i32,
    boss_damage: i32,
    shield_timer: i32,
    poison_timer: i32,
    recharge_timer: i32,
    mp_spent: i32,
}

impl State {
    fn new(hp: i32, mp: i32, boss_hp: i32, boss_damage: i32) -> Self {
        State {
            hp,
            mp,
            boss_hp,
            boss_damage,
            shield_timer: 0,
            poison_timer: 0,
            recharge_timer: 0,
            mp_spent: 0,
        }
    }

    fn start_turn(&mut self) {
        if self.shield_timer > 0 {
            self.shield_timer -= 1;
        }
        if self.poison_timer > 0 {
            self.poison_timer -= 1;
            self.boss_hp -= 3;
        }
        if self.recharge_timer > 0 {
            self.recharge_timer -= 1;
            self.mp += 101;
        }
    }

    fn boss_turn(&mut self) {
        let armor = if self.shield_timer > 0 { 7 } else { 0 };
        let damage = std::cmp::max(self.boss_damage - armor, 1);
        self.hp -= damage;
    }

    fn cast_magic_missile(&self) -> Option<Self> {
        const MP_COST: i32 = 53;
        if self.mp < MP_COST {
            None
        } else {
            let mut state = *self;
            state.mp -= MP_COST;
            state.mp_spent += MP_COST;
            state.boss_hp -= 4;
            Some(state)
        }
    }

    fn cast_drain(&self) -> Option<Self> {
        const MP_COST: i32 = 73;
        if self.mp < MP_COST {
            None
        } else {
            let mut state = *self;
            state.mp -= MP_COST;
            state.mp_spent += MP_COST;
            state.boss_hp -= 2;
            state.hp += 2;
            Some(state)
        }
    }

    fn cast_shield(&self) -> Option<Self> {
        const MP_COST: i32 = 113;
        if self.mp < MP_COST || self.shield_timer > 0 {
            None
        } else {
            let mut state = *self;
            state.mp -= MP_COST;
            state.mp_spent += MP_COST;
            state.shield_timer = 6;
            Some(state)
        }
    }

    fn cast_poison(&self) -> Option<Self> {
        const MP_COST: i32 = 173;
        if self.mp < MP_COST || self.poison_timer > 0 {
            None
        } else {
            let mut state = *self;
            state.mp -= MP_COST;
            state.mp_spent += MP_COST;
            state.poison_timer = 6;
            Some(state)
        }
    }

    fn cast_recharge(&self) -> Option<Self> {
        const MP_COST: i32 = 229;
        if self.mp < MP_COST || self.recharge_timer > 0 {
            None
        } else {
            let mut state = *self;
            state.mp -= MP_COST;
            state.mp_spent += MP_COST;
            state.recharge_timer = 5;
            Some(state)
        }
    }

    fn player_turn(&self) -> Vec<Self> {
        let mut new_states = Vec::<Self>::new();
        for action in [
            Self::cast_magic_missile,
            Self::cast_drain,
            Self::cast_shield,
            Self::cast_poison,
            Self::cast_recharge,
        ] {
            if let Some(state) = action(self) {
                new_states.push(state);
            }
        }
        new_states
    }

    fn step(states: &mut [Self], hard: bool) -> Either<Vec<Self>, Self> {
        let mut new_states = Vec::<Self>::new();
        for s in states.iter_mut() {
            if hard {
                s.hp -= 1;
                if s.hp <= 0 {
                    continue;
                }
            }
            s.start_turn();
            if s.boss_hp <= 0 {
                return Right(*s);
            }

            let mut outcomes = s.player_turn();
            for o in outcomes.iter_mut() {
                o.start_turn();
                if o.boss_hp <= 0 {
                    return Right(*o);
                }
                o.boss_turn();
                if o.hp > 0 {
                    new_states.push(*o);
                }
            }
        }
        Left(new_states)
    }
}

fn fight_with_least_mp_spent(state: &State, hard: bool) -> Option<i32> {
    let mut states = vec![*state];
    loop {
        let step_result = State::step(&mut states, hard);
        match step_result {
            Left(new_states) => {
                if new_states.is_empty() {
                    return None;
                }
                states = new_states;
            }
            Right(winning_state) => {
                return Some(winning_state.mp_spent);
            }
        }
    }
}

pub fn solve(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Failed to read file.");
    let (boss_hp, boss_damage) = parser()
        .parse(&input)
        .into_result()
        .expect("Failed to parse input");
    let state = State::new(50, 500, boss_hp, boss_damage);
    assert!(fight_with_least_mp_spent(&state, false) == Some(1824));
    assert!(fight_with_least_mp_spent(&state, true) == Some(1937));
}
