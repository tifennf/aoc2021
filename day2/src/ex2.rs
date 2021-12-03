use crate::{parse_instruction, Instruction};

struct SubmarineState {
    hor_pos: i32,
    aim: i32,
    depth: i32,
}

impl Default for SubmarineState {
    fn default() -> Self {
        Self {
            hor_pos: Default::default(),
            depth: Default::default(),
            aim: Default::default(),
        }
    }
}

pub fn run(inputs: &str) {
    let mut sub_state = SubmarineState::default();

    for instruction in inputs.lines() {
        let ins = parse_instruction(instruction);

        match ins {
            Instruction::Forward(value) => {
                sub_state.hor_pos += value as i32;

                let dept = sub_state.aim * value as i32;

                sub_state.depth += dept;
            }
            Instruction::Down(value) => sub_state.aim += value as i32,
            Instruction::Up(value) => sub_state.aim -= value as i32,
        }
    }

    let depth = sub_state.depth;
    let hor_pos = sub_state.hor_pos;

    let result = depth * hor_pos;

    println!(
        "Dept: {}\nHorizontal position: {}\nResult: {}",
        depth, hor_pos, result
    );
}
