use crate::{parse_instruction, Instruction};

struct SubmarineState {
    forward: i32,
    depth: i32,
}

impl Default for SubmarineState {
    fn default() -> Self {
        Self {
            forward: Default::default(),
            depth: Default::default(),
        }
    }
}

pub fn run(inputs: &str) {
    let mut sub_state = SubmarineState::default();

    for instruction in inputs.lines() {
        let ins = parse_instruction(instruction);

        match ins {
            Instruction::Forward(value) => sub_state.forward += value as i32,
            Instruction::Down(value) => sub_state.depth += value as i32,
            Instruction::Up(value) => sub_state.depth -= value as i32,
        }
    }

    let SubmarineState { forward, depth } = sub_state;

    let result = depth * forward;

    println!("Dept: {}\nForward: {}\nResult: {}", depth, forward, result);
}
