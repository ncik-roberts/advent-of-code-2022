use std::collections::VecDeque;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[0..4] {
            "noop" => Ok(Self::Noop),
            "addx" => {
                let arg = s[5..].parse()?;
                Ok(Self::Addx(arg))
            }
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct State {
    next_cycle: u32,
    read_next_instruction_at_cycle: u32,
    pending_addx: Option<i32>,
    instructions: VecDeque<Instruction>,
    register: i32,
}

impl State {
    fn new(instructions: VecDeque<Instruction>) -> Self {
        State {
            next_cycle: 1,
            read_next_instruction_at_cycle: 1,
            pending_addx: None,
            instructions,
            register: 1,
        }
    }

    fn tick(&mut self) {
        if self.next_cycle == self.read_next_instruction_at_cycle {
            match self.pending_addx {
                Some(addx) => {
                    self.register += addx;
                    self.pending_addx = None
                }
                None => (),
            };
            match self.instructions[0] {
                Instruction::Noop => self.read_next_instruction_at_cycle += 1,
                Instruction::Addx(i) => {
                    self.pending_addx = Some(i);
                    self.read_next_instruction_at_cycle += 2;
                }
            }
            self.instructions.pop_front();
        }

        self.next_cycle += 1;
    }
}

fn part1(lines: advent::LineStream) {
    let instructions = VecDeque::from_iter(lines.map(|l| l.parse::<Instruction>().unwrap()));
    let mut state = State::new(instructions);
    let mut sum_of_relevant_signal_strengths: i32 = 0;
    loop {
        let curr_cycle = state.next_cycle as i32;
        state.tick();
        if curr_cycle % 40 == 20 {
            let signal_strength = state.register * curr_cycle;
            sum_of_relevant_signal_strengths += signal_strength
        }
        if curr_cycle == 220 {
            break;
        }
    }
    println!("{sum_of_relevant_signal_strengths}")
}

fn part2(lines: advent::LineStream) {
    let instructions = VecDeque::from_iter(lines.map(|l| l.parse::<Instruction>().unwrap()));
    let mut state = State::new(instructions);
    println!();
    for _ in 0..6 {
        for pos in 0..40 {
            state.tick();
            if pos + 1 >= state.register && pos + 1 < state.register + 3 {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!();
    }
}

fn main() {
    advent::of_code(advent::PARSE_AS_STREAM, part1, part2)
}
