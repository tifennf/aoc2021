pub mod ex1;
pub mod ex2;
pub enum Instruction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

pub fn parse_instruction(ins: &str) -> Instruction {
    if ins.starts_with("forward") {
        let ins: Vec<_> = ins.split_whitespace().collect();

        let value = ins[1].trim().parse::<u32>().unwrap();

        Instruction::Forward(value)
    } else if ins.starts_with("down") {
        let ins: Vec<_> = ins.split_whitespace().collect();

        let value = ins[1].trim().parse::<u32>().unwrap();
        Instruction::Down(value)
    } else {
        let ins: Vec<_> = ins.split_whitespace().collect();

        let value = ins[1].trim().parse::<u32>().unwrap();
        Instruction::Up(value)
    }
}
